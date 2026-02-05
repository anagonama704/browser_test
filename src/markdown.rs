use crate::model::{Block, BlockKind, Document};

pub fn parse_markdown(input: &str) -> Document {
    let mut blocks = Vec::new();
    let mut paragraph_lines: Vec<String> = Vec::new();
    let mut in_code = false;
    let mut code_info: Option<String> = None;
    let mut code_lines: Vec<String> = Vec::new();
    let mut list_id_counter: usize = 0;
    let mut in_list = false;

    for line in input.lines() {
        let trimmed = line.trim_end();

        if in_code {
            if trimmed.starts_with("```") {
                let raw = code_lines.join("\n");
                let text = raw.clone();
                let index = blocks.len();
                blocks.push(Block::new(
                    BlockKind::CodeBlock {
                        info: code_info.take(),
                    },
                    text,
                    raw,
                    index,
                ));
                code_lines.clear();
                in_code = false;
            } else {
                code_lines.push(trimmed.to_string());
            }
            continue;
        }

        if trimmed.starts_with("```") {
            flush_paragraph(&mut blocks, &mut paragraph_lines);
            in_code = true;
            let info = trimmed.trim_start_matches("```").trim();
            if info.is_empty() {
                code_info = None;
            } else {
                code_info = Some(info.to_string());
            }
            in_list = false;
            continue;
        }

        if trimmed.trim().is_empty() {
            flush_paragraph(&mut blocks, &mut paragraph_lines);
            in_list = false;
            continue;
        }

        if let Some((level, text)) = parse_heading(trimmed) {
            flush_paragraph(&mut blocks, &mut paragraph_lines);
            let raw = trimmed.to_string();
            let text = strip_inline_md(text);
            let index = blocks.len();
            blocks.push(Block::new(BlockKind::Heading { level }, text, raw, index));
            in_list = false;
            continue;
        }

        if let Some((ordered, text)) = parse_list_item(trimmed) {
            flush_paragraph(&mut blocks, &mut paragraph_lines);
            if !in_list {
                list_id_counter = list_id_counter.saturating_add(1);
            }
            in_list = true;
            let raw = trimmed.to_string();
            let text = strip_inline_md(text);
            let index = blocks.len();
            blocks.push(Block::new(
                BlockKind::ListItem {
                    ordered,
                    list_id: list_id_counter,
                },
                text,
                raw,
                index,
            ));
            continue;
        }

        if let Some(text) = parse_blockquote(trimmed) {
            flush_paragraph(&mut blocks, &mut paragraph_lines);
            let raw = trimmed.to_string();
            let text = strip_inline_md(text);
            let index = blocks.len();
            blocks.push(Block::new(BlockKind::BlockQuote, text, raw, index));
            in_list = false;
            continue;
        }

        paragraph_lines.push(trimmed.to_string());
        in_list = false;
    }

    if in_code {
        let raw = code_lines.join("\n");
        let text = raw.clone();
        let index = blocks.len();
        blocks.push(Block::new(
            BlockKind::CodeBlock {
                info: code_info.take(),
            },
            text,
            raw,
            index,
        ));
    } else {
        flush_paragraph(&mut blocks, &mut paragraph_lines);
    }

    Document::new(input.to_string(), blocks)
}

fn flush_paragraph(blocks: &mut Vec<Block>, paragraph_lines: &mut Vec<String>) {
    if paragraph_lines.is_empty() {
        return;
    }
    let raw = paragraph_lines.join("\n");
    let text = strip_inline_md(&raw);
    let index = blocks.len();
    blocks.push(Block::new(BlockKind::Paragraph, text, raw, index));
    paragraph_lines.clear();
}

fn parse_heading(line: &str) -> Option<(u8, &str)> {
    let mut level: u8 = 0;
    let mut byte_index: usize = 0;
    for ch in line.chars() {
        if ch == '#' {
            level = level.saturating_add(1);
            byte_index += ch.len_utf8();
        } else {
            break;
        }
    }
    if level == 0 {
        return None;
    }
    let rest = line[byte_index..].trim();
    if rest.is_empty() {
        return None;
    }
    Some((level, rest))
}

fn parse_list_item(line: &str) -> Option<(bool, &str)> {
    let trimmed = line.trim_start();
    if let Some(rest) = trimmed.strip_prefix("- ") {
        return Some((false, rest));
    }
    if let Some(rest) = trimmed.strip_prefix("* ") {
        return Some((false, rest));
    }
    if let Some(rest) = trimmed.strip_prefix("+ ") {
        return Some((false, rest));
    }

    let bytes = trimmed.as_bytes();
    let mut idx = 0;
    while idx < bytes.len() && bytes[idx].is_ascii_digit() {
        idx += 1;
    }
    if idx > 0 && idx + 1 < bytes.len() && bytes[idx] == b'.' && bytes[idx + 1] == b' ' {
        let rest = &trimmed[idx + 2..];
        return Some((true, rest));
    }
    None
}

fn parse_blockquote(line: &str) -> Option<&str> {
    let trimmed = line.trim_start();
    if let Some(rest) = trimmed.strip_prefix('>') {
        return Some(rest.trim_start());
    }
    None
}

fn strip_inline_md(raw: &str) -> String {
    let mut out = String::new();
    let mut chars = raw.chars().peekable();
    while let Some(ch) = chars.next() {
        match ch {
            '*' | '_' | '`' => {}
            '!' => {
                if matches!(chars.peek(), Some('[')) {
                    chars.next();
                    let alt = collect_until(&mut chars, ']');
                    skip_link_target(&mut chars);
                    if !alt.is_empty() {
                        out.push_str(&alt);
                    }
                } else {
                    out.push('!');
                }
            }
            '[' => {
                let label = collect_until(&mut chars, ']');
                skip_link_target(&mut chars);
                out.push_str(&label);
            }
            _ => out.push(ch),
        }
    }
    out
}

fn collect_until(chars: &mut std::iter::Peekable<std::str::Chars<'_>>, end: char) -> String {
    let mut collected = String::new();
    while let Some(ch) = chars.next() {
        if ch == end {
            break;
        }
        collected.push(ch);
    }
    collected
}

fn skip_link_target(chars: &mut std::iter::Peekable<std::str::Chars<'_>>) {
    if matches!(chars.peek(), Some('(')) {
        chars.next();
        while let Some(ch) = chars.next() {
            if ch == ')' {
                break;
            }
        }
    }
}
