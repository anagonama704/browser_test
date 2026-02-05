//! ルールの基本インターフェース

use integrity_core::{ConstraintId, Evidence, Violation};
use integrity_parser::Document;

/// ルールのマッチ結果
#[derive(Debug, Clone)]
pub struct RuleMatch {
    /// マッチした文字列
    pub matched: String,
    /// 行番号（0-indexed）
    pub line: usize,
    /// コンテキスト
    pub context: Option<String>,
}

/// ルールトレイト
pub trait Rule: Send + Sync {
    /// このルールが対応する制約ID
    fn constraint_id(&self) -> ConstraintId;

    /// ドキュメントをチェックし、違反があればViolationを返す
    fn check(&self, doc: &Document) -> Vec<Violation>;
}

/// 語彙シグナルベースのルールを作成するヘルパー
pub fn create_violation_from_matches(
    constraint_id: ConstraintId,
    matches: Vec<RuleMatch>,
) -> Option<Violation> {
    if matches.is_empty() {
        return None;
    }

    let snippets: Vec<String> = matches.iter().map(|m| m.matched.clone()).collect();

    let line = matches.first().map(|m| m.line);
    let context = matches.first().and_then(|m| m.context.clone());

    let mut evidence = Evidence::new(snippets);
    if let Some(l) = line {
        evidence = evidence.with_line(l);
    }
    if let Some(c) = context {
        evidence = evidence.with_context(c);
    }

    Some(Violation::with_default_suggestion(constraint_id, evidence))
}
