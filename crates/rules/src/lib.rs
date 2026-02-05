//! integrity-rules: ハード制約のルール実装
//!
//! このクレートは、制約①〜⑦のチェックロジックを実装します。
//! ルールはデータ駆動で、将来的な拡張が可能な設計です。

mod checker;
mod rule;
mod rules;

pub use checker::ConstraintChecker;
pub use rule::{Rule, RuleMatch};

/// 全てのルールを適用してドキュメントをチェック
pub fn check_document(doc: &integrity_parser::Document) -> integrity_core::Report {
    let checker = ConstraintChecker::new();
    checker.check(doc)
}

/// テキストを直接チェック（パース込み）
pub fn check_text(input: &str) -> integrity_core::Report {
    let doc = integrity_parser::parse(input);
    check_document(&doc)
}
