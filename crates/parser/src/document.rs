//! ドキュメント（ブロックの集合）の定義

use crate::block::Block;

/// パースされたドキュメント
#[derive(Debug, Clone)]
pub struct Document {
    /// 元のテキスト
    pub raw_text: String,
    /// ブロックのリスト
    pub blocks: Vec<Block>,
}

impl Document {
    /// テキストをパースしてDocumentを生成
    pub fn parse(input: &str) -> Self {
        let raw_text = input.to_string();
        let blocks = Self::split_into_blocks(input);

        Self { raw_text, blocks }
    }

    /// テキストをブロックに分割
    fn split_into_blocks(input: &str) -> Vec<Block> {
        let mut blocks = Vec::new();
        let mut current_block = String::new();
        let mut current_line_number = 0;
        let mut block_start_line = 0;

        for (line_idx, line) in input.lines().enumerate() {
            let trimmed = line.trim();

            // 空行でブロックを区切る
            if trimmed.is_empty() {
                if !current_block.is_empty() {
                    blocks.push(Block::new(
                        current_block.trim().to_string(),
                        block_start_line,
                    ));
                    current_block.clear();
                }
                current_line_number = line_idx + 1;
                continue;
            }

            // 新しいブロックの開始を検出
            let is_new_block_start = trimmed.starts_with('#')
                || trimmed.starts_with('【')
                || trimmed.starts_with('-')
                || trimmed.starts_with('・')
                || trimmed.starts_with('•')
                || trimmed.starts_with("* ")
                || trimmed.starts_with('※')
                || trimmed.chars().next().is_some_and(|c| c.is_ascii_digit())
                    && trimmed.chars().nth(1).is_some_and(|c| c == '.' || c == ')');

            if is_new_block_start && !current_block.is_empty() {
                blocks.push(Block::new(
                    current_block.trim().to_string(),
                    block_start_line,
                ));
                current_block.clear();
                block_start_line = line_idx;
            }

            if current_block.is_empty() {
                block_start_line = line_idx;
            }

            if !current_block.is_empty() {
                current_block.push(' ');
            }
            current_block.push_str(trimmed);
            current_line_number = line_idx;
        }

        // 最後のブロックを追加
        if !current_block.is_empty() {
            blocks.push(Block::new(
                current_block.trim().to_string(),
                block_start_line,
            ));
        }

        // 空の場合は全体を1つのブロックとして扱う（行番号の整合性のため）
        let _ = current_line_number;

        blocks
    }

    /// 全てのブロックのテキストを結合
    pub fn full_text(&self) -> &str {
        &self.raw_text
    }

    /// ブロック数を取得
    pub fn block_count(&self) -> usize {
        self.blocks.len()
    }

    /// 指定した行番号を含むブロックを取得
    pub fn block_at_line(&self, line: usize) -> Option<&Block> {
        self.blocks
            .iter()
            .find(|b| b.line_number <= line && line < b.line_number + b.text.lines().count())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple() {
        let doc = Document::parse("これはテスト文です。\n\n別の段落です。");
        assert_eq!(doc.block_count(), 2);
    }

    #[test]
    fn test_parse_with_headings() {
        let doc = Document::parse("# 見出し\n\nこれは本文です。");
        assert_eq!(doc.block_count(), 2);
    }

    #[test]
    fn test_parse_with_list() {
        let doc = Document::parse("- 項目1\n- 項目2\n- 項目3");
        assert_eq!(doc.block_count(), 3);
    }
}
