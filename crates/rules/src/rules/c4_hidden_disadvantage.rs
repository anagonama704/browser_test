//! 制約④ 不利情報隠しの禁止
//!
//! ユーザーにとって不利な情報を、目立たない場所に配置したり、アクセスしにくくすることを禁止する。

use integrity_core::{ConstraintId, Violation};
use integrity_parser::Document;
use once_cell::sync::Lazy;
use regex::Regex;

use crate::rule::{create_violation_from_matches, Rule, RuleMatch};

/// 不利情報隠し禁止ルール
pub struct HiddenDisadvantageRule {
    patterns: Vec<Regex>,
}

impl HiddenDisadvantageRule {
    /// 新しいルールを作成
    pub fn new() -> Self {
        // v0の仮定: 以下の語彙シグナルで不利情報隠しを検出
        static PATTERNS: Lazy<Vec<Regex>> = Lazy::new(|| {
            vec![
                Regex::new(r"※\s*詳細").expect("invalid regex"),
                Regex::new(r"(?i)別ページに記載").expect("invalid regex"),
                Regex::new(r"(?i)利用規約(に|を)(記載|ご確認|参照)").expect("invalid regex"),
                Regex::new(r"(?i)小さく書いて").expect("invalid regex"),
                Regex::new(r"(?i)注釈(を)?参照").expect("invalid regex"),
                Regex::new(r"(?i)詳しくはこちら").expect("invalid regex"),
                Regex::new(r"(?i)詳細は(別途|後日|別ページ)").expect("invalid regex"),
                Regex::new(r"(?i)条件(は|については).*参照").expect("invalid regex"),
                Regex::new(r"※[^。]{0,20}(条件|詳細|規約)").expect("invalid regex"),
                Regex::new(r"(?i)\*\s*(条件|詳細|適用)").expect("invalid regex"),
            ]
        });

        Self {
            patterns: PATTERNS.clone(),
        }
    }
}

impl Default for HiddenDisadvantageRule {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for HiddenDisadvantageRule {
    fn constraint_id(&self) -> ConstraintId {
        ConstraintId::HIDDEN_DISADVANTAGE
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
    fn test_detect_hidden_disadvantage() {
        let doc =
            integrity_parser::parse("月額980円 ※解約手数料については利用規約をご確認ください");
        let rule = HiddenDisadvantageRule::new();
        let violations = rule.check(&doc);
        assert!(!violations.is_empty());
    }

    #[test]
    fn test_no_false_positive() {
        let doc = integrity_parser::parse("月額980円（解約時に手数料3,000円が発生します）");
        let rule = HiddenDisadvantageRule::new();
        let violations = rule.check(&doc);
        assert!(violations.is_empty());
    }
}
