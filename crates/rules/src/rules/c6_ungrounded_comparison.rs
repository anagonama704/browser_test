//! 制約⑥ 根拠のない比較・優位性の禁止
//!
//! 調査期間、母数、出典を明示せずに、No.1や最安等の優位性を主張することを禁止する。

use integrity_core::{ConstraintId, Evidence, Violation};
use integrity_parser::Document;
use once_cell::sync::Lazy;
use regex::Regex;

use crate::rule::Rule;

/// 根拠なき比較禁止ルール
pub struct UngroundedComparisonRule {
    claim_patterns: Vec<Regex>,
    evidence_patterns: Vec<Regex>,
}

impl UngroundedComparisonRule {
    /// 新しいルールを作成
    pub fn new() -> Self {
        // v0の仮定: 優位性の主張パターン
        static CLAIM_PATTERNS: Lazy<Vec<Regex>> = Lazy::new(|| {
            vec![
                Regex::new(r"(?i)No\.?\s*1").expect("invalid regex"),
                Regex::new(r"(?i)ナンバーワン").expect("invalid regex"),
                Regex::new(r"(?i)業界一").expect("invalid regex"),
                Regex::new(r"(?i)最安").expect("invalid regex"),
                Regex::new(r"(?i)一番人気").expect("invalid regex"),
                Regex::new(r"(?i)トップクラス").expect("invalid regex"),
                Regex::new(r"(?i)業界最高").expect("invalid regex"),
                Regex::new(r"(?i)日本一").expect("invalid regex"),
                Regex::new(r"(?i)世界一").expect("invalid regex"),
                Regex::new(r"(?i)最大級").expect("invalid regex"),
                Regex::new(r"(?i)首位").expect("invalid regex"),
                Regex::new(r"(?i)シェアNo").expect("invalid regex"),
            ]
        });

        // v0の仮定: 根拠を示すパターン
        static EVIDENCE_PATTERNS: Lazy<Vec<Regex>> = Lazy::new(|| {
            vec![
                Regex::new(r"調査").expect("invalid regex"),
                Regex::new(r"期間").expect("invalid regex"),
                Regex::new(r"(?i)n\s*=\s*\d+").expect("invalid regex"),
                Regex::new(r"母数").expect("invalid regex"),
                Regex::new(r"サンプル").expect("invalid regex"),
                Regex::new(r"出典").expect("invalid regex"),
                Regex::new(r"(?i)\d{4}年").expect("invalid regex"),
                Regex::new(r"調査会社").expect("invalid regex"),
                Regex::new(r"リサーチ").expect("invalid regex"),
            ]
        });

        Self {
            claim_patterns: CLAIM_PATTERNS.clone(),
            evidence_patterns: EVIDENCE_PATTERNS.clone(),
        }
    }

    /// ブロック内で優位性主張を検出
    fn find_claims(&self, text: &str) -> Vec<String> {
        let mut claims = Vec::new();
        for pattern in &self.claim_patterns {
            for m in pattern.find_iter(text) {
                claims.push(m.as_str().to_string());
            }
        }
        claims
    }

    /// ブロック内で根拠を検出
    fn has_evidence(&self, text: &str) -> bool {
        for pattern in &self.evidence_patterns {
            if pattern.is_match(text) {
                return true;
            }
        }
        false
    }
}

impl Default for UngroundedComparisonRule {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for UngroundedComparisonRule {
    fn constraint_id(&self) -> ConstraintId {
        ConstraintId::UNGROUNDED_COMPARISON
    }

    fn check(&self, doc: &Document) -> Vec<Violation> {
        let mut violations = Vec::new();

        for block in &doc.blocks {
            let claims = self.find_claims(&block.text);

            // 優位性の主張があり、かつ根拠がない場合は違反
            if !claims.is_empty() && !self.has_evidence(&block.text) {
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
    fn test_detect_ungrounded_comparison() {
        let doc = integrity_parser::parse("顧客満足度No.1！");
        let rule = UngroundedComparisonRule::new();
        let violations = rule.check(&doc);
        assert!(!violations.is_empty());
    }

    #[test]
    fn test_no_false_positive_with_evidence() {
        let doc = integrity_parser::parse("顧客満足度No.1（○○調査会社、2024年1-3月、n=10,000）");
        let rule = UngroundedComparisonRule::new();
        let violations = rule.check(&doc);
        assert!(violations.is_empty());
    }
}
