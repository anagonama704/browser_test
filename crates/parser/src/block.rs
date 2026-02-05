//! ブロック（段落/文）の定義

use once_cell::sync::Lazy;
use regex::Regex;

/// ブロックの種類
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlockKind {
    /// 通常の段落
    Paragraph,
    /// 見出し
    Heading,
    /// リスト項目
    ListItem,
    /// 注釈・補足
    Note,
    /// その他
    Other,
}

/// テキストブロック
#[derive(Debug, Clone)]
pub struct Block {
    /// ブロックの種類
    pub kind: BlockKind,
    /// テキスト内容
    pub text: String,
    /// 元のテキストでの行番号（0-indexed）
    pub line_number: usize,
    /// ブロック内の文のリスト
    pub sentences: Vec<String>,
}

impl Block {
    /// 新しいブロックを作成
    pub fn new(text: String, line_number: usize) -> Self {
        let kind = Self::detect_kind(&text);
        let sentences = Self::split_sentences(&text);
        Self {
            kind,
            text,
            line_number,
            sentences,
        }
    }

    /// ブロックの種類を検出
    fn detect_kind(text: &str) -> BlockKind {
        let trimmed = text.trim();

        // 見出しの検出（#で始まる、または【】で囲まれた短いテキスト）
        if trimmed.starts_with('#') || (trimmed.starts_with('【') && trimmed.len() < 50) {
            return BlockKind::Heading;
        }

        // リスト項目の検出
        if trimmed.starts_with('-')
            || trimmed.starts_with('・')
            || trimmed.starts_with('•')
            || trimmed.starts_with("* ")
        {
            return BlockKind::ListItem;
        }

        // 番号付きリストの検出
        static NUMBERED_LIST: Lazy<Regex> =
            Lazy::new(|| Regex::new(r"^\d+[\.\)]\s").expect("invalid regex"));
        if NUMBERED_LIST.is_match(trimmed) {
            return BlockKind::ListItem;
        }

        // 注釈の検出
        if trimmed.starts_with('※') || trimmed.starts_with('＊') || trimmed.starts_with("注:") {
            return BlockKind::Note;
        }

        BlockKind::Paragraph
    }

    /// テキストを文に分割
    fn split_sentences(text: &str) -> Vec<String> {
        // 句点や改行で分割
        static SENTENCE_SPLIT: Lazy<Regex> =
            Lazy::new(|| Regex::new(r"[。！？\n]+").expect("invalid regex"));

        SENTENCE_SPLIT
            .split(text)
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .map(String::from)
            .collect()
    }

    /// ブロックに指定したパターンが含まれるかチェック
    pub fn contains_pattern(&self, pattern: &Regex) -> bool {
        pattern.is_match(&self.text)
    }

    /// ブロック内でパターンにマッチする部分を全て取得
    pub fn find_all_matches(&self, pattern: &Regex) -> Vec<String> {
        pattern
            .find_iter(&self.text)
            .map(|m| m.as_str().to_string())
            .collect()
    }
}
