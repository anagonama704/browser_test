//! 制約② 視覚格差の禁止
//!
//! 選択肢間で視覚的な差異（サイズ、色、配置）を設けて特定の選択を誘導することを禁止する。

use integrity_core::{ConstraintId, Violation};
use integrity_parser::Document;
use once_cell::sync::Lazy;
use regex::Regex;

use crate::rule::{create_violation_from_matches, Rule, RuleMatch};

/// 視覚格差禁止ルール
pub struct VisualBiasRule {
    patterns: Vec<Regex>,
}

impl VisualBiasRule {
    /// 新しいルールを作成
    pub fn new() -> Self {
        // v0の仮定: 以下の語彙シグナルで視覚格差を検出
        static PATTERNS: Lazy<Vec<Regex>> = Lazy::new(|| {
            vec![
                Regex::new(r"(?i)目立つボタン").expect("invalid regex"),
                Regex::new(r"(?i)赤い(ボタン|文字|色)").expect("invalid regex"),
                Regex::new(r"(?i)大きく(表示|見せる)").expect("invalid regex"),
                Regex::new(r"(?i)小さく(表示|見せる)").expect("invalid regex"),
                Regex::new(r"(?i)グレーアウト").expect("invalid regex"),
                Regex::new(r"(?i)目立たない(ように|色|位置|場所)").expect("invalid regex"),
                Regex::new(r"(?i)薄い(色|グレー)で表示").expect("invalid regex"),
                Regex::new(r"(?i)強調(表示|する|して)").expect("invalid regex"),
                Regex::new(r"(?i)太字で(表示|強調)").expect("invalid regex"),
                Regex::new(r"(?i)色を変え(て|る)").expect("invalid regex"),
                Regex::new(r"(?i)サイズを(大きく|小さく)").expect("invalid regex"),
                Regex::new(r"(?i)目立(つ|た)せ(る|ない)").expect("invalid regex"),
            ]
        });

        Self {
            patterns: PATTERNS.clone(),
        }
    }
}

impl Default for VisualBiasRule {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for VisualBiasRule {
    fn constraint_id(&self) -> ConstraintId {
        ConstraintId::VISUAL_BIAS
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
    fn test_detect_visual_bias() {
        let doc = integrity_parser::parse(
            "「購入する」を大きく赤いボタンで、「キャンセル」を小さくグレーで表示",
        );
        let rule = VisualBiasRule::new();
        let violations = rule.check(&doc);
        assert!(!violations.is_empty());
    }

    #[test]
    fn test_no_false_positive() {
        let doc =
            integrity_parser::parse("「購入する」「キャンセル」を同じサイズ・同じ色で並列表示");
        let rule = VisualBiasRule::new();
        let violations = rule.check(&doc);
        assert!(violations.is_empty());
    }
}
