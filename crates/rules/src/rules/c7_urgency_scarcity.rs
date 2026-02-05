//! 制約⑦ 根拠のない急がせ・希少性の禁止
//!
//! 実際の期限や在庫状況に基づかない緊急性・希少性の演出を禁止する。

use integrity_core::{ConstraintId, Evidence, Violation};
use integrity_parser::Document;
use once_cell::sync::Lazy;
use regex::Regex;

use crate::rule::Rule;

/// 急がせ希少性禁止ルール
pub struct UrgencyScarcityRule {
    urgency_patterns: Vec<Regex>,
    justification_patterns: Vec<Regex>,
}

impl UrgencyScarcityRule {
    /// 新しいルールを作成
    pub fn new() -> Self {
        // v0の仮定: 緊急性・希少性を示すパターン
        static URGENCY_PATTERNS: Lazy<Vec<Regex>> = Lazy::new(|| {
            vec![
                Regex::new(r"(?i)今だけ").expect("invalid regex"),
                Regex::new(r"(?i)残りわずか").expect("invalid regex"),
                Regex::new(r"(?i)あと\d+[分時間日]").expect("invalid regex"),
                Regex::new(r"(?i)限定").expect("invalid regex"),
                Regex::new(r"(?i)本日限り").expect("invalid regex"),
                Regex::new(r"(?i)先着\d*名").expect("invalid regex"),
                Regex::new(r"(?i)在庫僅少").expect("invalid regex"),
                Regex::new(r"(?i)売り切れ間近").expect("invalid regex"),
                Regex::new(r"(?i)お急ぎ").expect("invalid regex"),
                Regex::new(r"(?i)今すぐ").expect("invalid regex"),
                Regex::new(r"(?i)期間限定").expect("invalid regex"),
                Regex::new(r"(?i)数量限定").expect("invalid regex"),
                Regex::new(r"(?i)早い者勝ち").expect("invalid regex"),
            ]
        });

        // v0の仮定: 根拠を示すパターン
        static JUSTIFICATION_PATTERNS: Lazy<Vec<Regex>> = Lazy::new(|| {
            vec![
                Regex::new(r"\d+月\d+日").expect("invalid regex"),
                Regex::new(r"\d+:\d+").expect("invalid regex"),
                Regex::new(r"(?i)在庫(残り)?\d+").expect("invalid regex"),
                Regex::new(r"(?i)残り\d+個").expect("invalid regex"),
                Regex::new(r"(?i)更新").expect("invalid regex"),
                Regex::new(r"(?i)\d+時(まで|迄)").expect("invalid regex"),
                Regex::new(r"(?i)〜\d+日").expect("invalid regex"),
                Regex::new(r"(?i)期限.*\d").expect("invalid regex"),
            ]
        });

        Self {
            urgency_patterns: URGENCY_PATTERNS.clone(),
            justification_patterns: JUSTIFICATION_PATTERNS.clone(),
        }
    }

    /// ブロック内で緊急性・希少性の主張を検出
    fn find_urgency_claims(&self, text: &str) -> Vec<String> {
        let mut claims = Vec::new();
        for pattern in &self.urgency_patterns {
            for m in pattern.find_iter(text) {
                claims.push(m.as_str().to_string());
            }
        }
        claims
    }

    /// ブロック内で根拠を検出
    fn has_justification(&self, text: &str) -> bool {
        for pattern in &self.justification_patterns {
            if pattern.is_match(text) {
                return true;
            }
        }
        false
    }
}

impl Default for UrgencyScarcityRule {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for UrgencyScarcityRule {
    fn constraint_id(&self) -> ConstraintId {
        ConstraintId::URGENCY_SCARCITY
    }

    fn check(&self, doc: &Document) -> Vec<Violation> {
        let mut violations = Vec::new();

        for block in &doc.blocks {
            let claims = self.find_urgency_claims(&block.text);

            // 緊急性・希少性の主張があり、かつ根拠がない場合は違反
            if !claims.is_empty() && !self.has_justification(&block.text) {
                let evidence = Evidence::new(claims)
                    .with_line(block.line_number)
                    .with_context(block.text.clone());

                violations.push(Violation::with_default_suggestion(
                    self.constraint_id(),
                    evidence,
                ));
            }
        }

        violations
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_urgency_scarcity() {
        let doc = integrity_parser::parse("今だけ特別価格！残りわずかです！");
        let rule = UrgencyScarcityRule::new();
        let violations = rule.check(&doc);
        assert!(!violations.is_empty());
    }

    #[test]
    fn test_no_false_positive_with_justification() {
        let doc =
            integrity_parser::parse("12月31日23:59までの期間限定価格（在庫残り23個、毎日0時更新）");
        let rule = UrgencyScarcityRule::new();
        let violations = rule.check(&doc);
        assert!(violations.is_empty());
    }
}
