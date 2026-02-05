//! 制約③ やめにくさの禁止
//!
//! 解約・退会・キャンセル等の手続きを、開始手続きより困難にすることを禁止する。

use integrity_core::{ConstraintId, Violation};
use integrity_parser::Document;
use once_cell::sync::Lazy;
use regex::Regex;

use crate::rule::{create_violation_from_matches, Rule, RuleMatch};

/// やめにくさ禁止ルール
pub struct HardToCancelRule {
    patterns: Vec<Regex>,
}

impl HardToCancelRule {
    /// 新しいルールを作成
    pub fn new() -> Self {
        // v0の仮定: 以下の語彙シグナルでやめにくさを検出
        static PATTERNS: Lazy<Vec<Regex>> = Lazy::new(|| {
            vec![
                Regex::new(r"(?i)解約は電話").expect("invalid regex"),
                Regex::new(r"(?i)解約(の|には|する場合).*(お問い合わせ|連絡|お電話)")
                    .expect("invalid regex"),
                Regex::new(r"(?i)(解約|退会|キャンセル).*(手続きが必要|手続きを)")
                    .expect("invalid regex"),
                Regex::new(r"(?i)解約は別(ページ|画面|サイト)").expect("invalid regex"),
                Regex::new(r"(?i)カスタマーサポート.*(連絡|お電話|ご連絡|まで)")
                    .expect("invalid regex"),
                Regex::new(r"(?i)(解約|退会).*電話.*のみ").expect("invalid regex"),
                Regex::new(r"(?i)書面(で|による)(解約|退会)").expect("invalid regex"),
                Regex::new(r"(?i)(解約|退会).*郵送").expect("invalid regex"),
                Regex::new(r"(?i)(平日|営業時間).*(解約|受付)").expect("invalid regex"),
                Regex::new(r"(?i)(解約|退会).*窓口.*お越し").expect("invalid regex"),
                Regex::new(r"(?i)(解約|退会).*(お電話|電話)").expect("invalid regex"),
            ]
        });

        Self {
            patterns: PATTERNS.clone(),
        }
    }
}

impl Default for HardToCancelRule {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for HardToCancelRule {
    fn constraint_id(&self) -> ConstraintId {
        ConstraintId::HARD_TO_CANCEL
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
    fn test_detect_hard_to_cancel() {
        let doc = integrity_parser::parse(
            "ご解約をご希望の場合は、カスタマーサポート（平日10-17時）までお電話ください",
        );
        let rule = HardToCancelRule::new();
        let violations = rule.check(&doc);
        assert!(!violations.is_empty());
    }

    #[test]
    fn test_no_false_positive() {
        let doc = integrity_parser::parse("解約ボタンをクリックすると即座に解約が完了します");
        let rule = HardToCancelRule::new();
        let violations = rule.check(&doc);
        assert!(violations.is_empty());
    }
}
