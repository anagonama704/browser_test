//! 制約チェッカー

use integrity_core::{Report, Violation};
use integrity_parser::Document;

use crate::rule::Rule;
use crate::rules::{
    FactOpinionMixRule, HardToCancelRule, HiddenDisadvantageRule, PreselectRule,
    UngroundedComparisonRule, UrgencyScarcityRule, VisualBiasRule,
};

/// 制約チェッカー
pub struct ConstraintChecker {
    rules: Vec<Box<dyn Rule>>,
}

impl ConstraintChecker {
    /// 全てのルールを含む新しいチェッカーを作成
    pub fn new() -> Self {
        let rules: Vec<Box<dyn Rule>> = vec![
            Box::new(PreselectRule::new()),
            Box::new(VisualBiasRule::new()),
            Box::new(HardToCancelRule::new()),
            Box::new(HiddenDisadvantageRule::new()),
            Box::new(FactOpinionMixRule::new()),
            Box::new(UngroundedComparisonRule::new()),
            Box::new(UrgencyScarcityRule::new()),
        ];

        Self { rules }
    }

    /// ドキュメントをチェックしてレポートを生成
    pub fn check(&self, doc: &Document) -> Report {
        let mut violations: Vec<Violation> = Vec::new();

        for rule in &self.rules {
            let rule_violations = rule.check(doc);
            violations.extend(rule_violations);
        }

        // 行番号でソート
        violations.sort_by(|a, b| {
            let line_a = a.evidence.line.unwrap_or(usize::MAX);
            let line_b = b.evidence.line.unwrap_or(usize::MAX);
            line_a.cmp(&line_b)
        });

        Report::new(violations)
    }
}

impl Default for ConstraintChecker {
    fn default() -> Self {
        Self::new()
    }
}
