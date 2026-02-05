//! integrity-parser: テキスト解析
//!
//! このクレートは、入力テキストを解析し、段落やブロックに分割します。
//! v0では簡易的な実装で、将来的にBlueprint等へ拡張可能な設計としています。

mod block;
mod document;

pub use block::{Block, BlockKind};
pub use document::Document;

/// 入力テキストをパースしてDocumentを生成
pub fn parse(input: &str) -> Document {
    Document::parse(input)
}
