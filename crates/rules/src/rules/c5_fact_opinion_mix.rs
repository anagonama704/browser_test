//! 制約⑤ 事実と意見の混在禁止
//!
//! 客観的事実（数値、データ）と主観的意見（おすすめ、評価）を同一段落で混在させ、
//! 区別を困難にすることを禁止する。

use integrity_core::{ConstraintId, Evidence, Violation};
use integrity_parser::Document;
use once_cell::sync::Lazy;
use regex::Regex;

use crate::rule::Rule;

/// 事実と意見の混在禁止ルール
pub struct FactOpinionMixRule {
    fact_patterns: Vec<Regex>,
    opinion_patterns: Vec<Regex>,
}

impl FactOpinionMixRule {
    /// 新しいルールを作成
    pub fn new() -> Self {
        // v0の仮定: 事実を示すパターン
        static FACT_PATTERNS: Lazy<Vec<Regex>> = Lazy::new(|| {
            vec![
                Regex::new(r"\d+%").expect("invalid regex"),
                Regex::new(r"\d+人").expect("invalid regex"),
                Regex::new(r"\d+件").expect("invalid regex"),
                Regex::new(r"調査(結果|では|によ)").expect("invalid regex"),
                Regex::new(r"統計").expect("invalid regex"),
                Regex::new(r"データ(によ|では)").expect("invalid regex"),
                Regex::new(r"実績").expect("invalid regex"),
                Regex::new(r"満足度\d+").expect("invalid regex"),
            ]
        });

        // v0の仮定: 意見を示すパターン
        static OPINION_PATTERNS: Lazy<Vec<Regex>> = Lazy::new(|| {
            vec![
                Regex::new(r"(?i)おすすめ").expect("invalid regex"),
                Regex::new(r"(?i)最高").expect("invalid regex"),
                Regex::new(r"(?i)今すぐ").expect("invalid regex"),
                Regex::new(r"(?i)ぜひ").expect("invalid regex"),
                Regex::new(r"(?i)絶対").expect("invalid regex"),
                Regex::new(r"(?i)必見").expect("invalid regex"),
                Regex::new(r"(?i)お見逃しなく").expect("invalid regex"),
                Regex::new(r"(?i)間違いない").expect("invalid regex"),
            ]
        });

        Self {
            fact_patterns: FACT_PATTERNS.clone(),
            opinion_patterns: OPINION_PATTERNS.clone(),
        }
    }

    /// ブロック内で事実パターンを検出
    fn find_facts(&self, text: &str) -> Vec<String> {
        let mut facts = Vec::new();
        for pattern in &self.fact_patterns {
            for m in pattern.find_iter(text) {
                facts.push(m.as_str().to_string());
            }
        }
        facts
    }

    /// ブロック内で意見パターンを検出
    fn find_opinions(&self, text: &str) -> Vec<String> {
        let mut opinions = Vec::new();
        for pattern in &self.opinion_patterns {
            for m in pattern.find_iter(text) {
                opinions.push(m.as_str().to_string());
            }
        }
        opinions
    }
}

impl Default for FactOpinionMixRule {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for FactOpinionMixRule {
    fn constraint_id(&self) -> ConstraintId {
        ConstraintId::FACT_OPINION_MIX
    }

    fn check(&self, doc: &Document) -> Vec<Violation> {
        let mut violations = Vec::new();

        for block in &doc.blocks {
            let facts = self.find_facts(&block.text);
            let opinions = self.find_opinions(&block.text);

            // 同一ブロック内に事実と意見が両方存在する場合は違反
            if !facts.is_empty() && !opinions.is_empty() {
                let mut snippets = facts;
                snippets.extend(opinions);

                let evidence = Evidence::new(snippets)
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
    fn test_detect_fact_opinion_mix() {
        let doc = integrity_parser::parse(
            "当社の満足度調査では92%のお客様が満足と回答。今すぐお申し込みがおすすめです！",
        );
        let rule = FactOpinionMixRule::new();
        let violations = rule.check(&doc);
        assert!(!violations.is_empty());
    }

    #[test]
    fn test_no_false_positive_separated() {
        let doc = integrity_parser::parse(
            "【調査結果】満足度調査では92%のお客様が満足と回答（2024年4月実施、n=500）\n\n【ご案内】お申し込みはこちらから",
        );
        let rule = FactOpinionMixRule::new();
        let violations = rule.check(&doc);
        // 別のブロックなので混在していない
        assert!(violations.is_empty());
    }
}
