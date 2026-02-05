//! 制約① プリセレクトの禁止
//!
//! ユーザーの明示的な意思なく、オプションが事前に選択されている状態を禁止する。

use integrity_core::{ConstraintId, Violation};
use integrity_parser::Document;
use once_cell::sync::Lazy;
use regex::Regex;

use crate::rule::{create_violation_from_matches, Rule, RuleMatch};

/// プリセレクト禁止ルール
pub struct PreselectRule {
    patterns: Vec<Regex>,
}

impl PreselectRule {
    /// 新しいルールを作成
    pub fn new() -> Self {
        // v0の仮定: 以下の語彙シグナルでプリセレクトを検出
        static PATTERNS: Lazy<Vec<Regex>> = Lazy::new(|| {
            vec![
                Regex::new(r"(?i)チェック済み").expect("invalid regex"),
                Regex::new(r"(?i)デフォルトで(ON|オン|有効)").expect("invalid regex"),
                Regex::new(r"(?i)初期選択").expect("invalid regex"),
                Regex::new(r"(?i)あらかじめ選択").expect("invalid regex"),
                Regex::new(r"(?i)事前に選択").expect("invalid regex"),
                Regex::new(r"(?i)checked\s+by\s+default").expect("invalid regex"),
                Regex::new(r"(?i)pre[-\s]?selected").expect("invalid regex"),
                Regex::new(r"(?i)default(ed)?\s+(to|is)\s+(on|selected|checked)")
                    .expect("invalid regex"),
                Regex::new(r"(?i)最初から(選択|チェック|ON)").expect("invalid regex"),
                Regex::new(r"(?i)自動的に(選択|チェック|追加)").expect("invalid regex"),
            ]
        });

        Self {
            patterns: PATTERNS.clone(),
        }
    }
}

impl Default for PreselectRule {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for PreselectRule {
    fn constraint_id(&self) -> ConstraintId {
        ConstraintId::PRESELECT
    }

    fn check(&self, doc: &Document) -> Vec<Violation> {
        let mut all_matches: Vec<RuleMatch> = Vec::new();

        for block in &doc.blocks {
            for pattern in &self.patterns {
                for m in pattern.find_iter(&block.text) {
                    all_matches.push(RuleMatch {
                        matched: m.as_str().to_string(),
                        line: block.line_number,
                        context: Some(block.text.clone()),
                    });
                }
            }
        }

        create_violation_from_matches(self.constraint_id(), all_matches)
            .into_iter()
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_preselect() {
        let doc = integrity_parser::parse("メールマガジン購読がデフォルトでONになっています");
        let rule = PreselectRule::new();
        let violations = rule.check(&doc);
        assert!(!violations.is_empty());
    }

    #[test]
    fn test_no_false_positive() {
        let doc = integrity_parser::parse("メールマガジンを購読しますか？ □ はい □ いいえ");
        let rule = PreselectRule::new();
        let violations = rule.check(&doc);
        assert!(violations.is_empty());
    }
}
