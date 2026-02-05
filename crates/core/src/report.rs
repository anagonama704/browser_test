//! レポートモデルの定義

use serde::{Deserialize, Serialize};

use crate::{ConstraintId, Severity};

/// レポートのバージョン
pub const REPORT_VERSION: &str = "0.1";

/// 違反の証拠
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Evidence {
    /// 違反が検出されたテキスト片
    pub snippets: Vec<String>,
    /// 違反が検出された位置（行番号、0-indexed）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line: Option<usize>,
    /// コンテキスト（前後のテキスト）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<String>,
}

impl Evidence {
    /// 新しい証拠を作成
    pub fn new(snippets: Vec<String>) -> Self {
        Self {
            snippets,
            line: None,
            context: None,
        }
    }

    /// 行番号を設定
    pub fn with_line(mut self, line: usize) -> Self {
        self.line = Some(line);
        self
    }

    /// コンテキストを設定
    pub fn with_context(mut self, context: String) -> Self {
        self.context = Some(context);
        self
    }
}

/// 単一の違反
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Violation {
    /// 制約ID
    pub constraint_id: ConstraintId,
    /// 制約タイトル
    pub title: String,
    /// 重大度
    pub severity: Severity,
    /// 証拠
    pub evidence: Evidence,
    /// 修正提案
    pub suggestions: Vec<String>,
}

impl Violation {
    /// 新しい違反を作成
    pub fn new(constraint_id: ConstraintId, evidence: Evidence, suggestions: Vec<String>) -> Self {
        Self {
            constraint_id,
            title: constraint_id.title().to_string(),
            severity: Severity::HardFail,
            evidence,
            suggestions,
        }
    }

    /// デフォルトの提案で違反を作成
    pub fn with_default_suggestion(constraint_id: ConstraintId, evidence: Evidence) -> Self {
        Self::new(
            constraint_id,
            evidence,
            vec![constraint_id.suggestion().to_string()],
        )
    }
}

/// チェックレポート
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Report {
    /// レポートバージョン
    pub version: String,
    /// 検出された違反のリスト
    pub violations: Vec<Violation>,
}

impl Report {
    /// 新しいレポートを作成
    pub fn new(violations: Vec<Violation>) -> Self {
        Self {
            version: REPORT_VERSION.to_string(),
            violations,
        }
    }

    /// 違反があるかどうか
    pub fn has_violations(&self) -> bool {
        !self.violations.is_empty()
    }

    /// 違反数を取得
    pub fn violation_count(&self) -> usize {
        self.violations.len()
    }

    /// JSON形式で出力
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }

    /// 人間が読みやすい形式で出力
    pub fn to_human_readable(&self) -> String {
        let mut output = String::new();

        if self.violations.is_empty() {
            output.push_str("✓ 違反は検出されませんでした。\n");
            return output;
        }

        output.push_str(&format!(
            "⚠ {} 件の違反が検出されました\n",
            self.violations.len()
        ));
        output.push_str("========================================\n\n");

        for (i, violation) in self.violations.iter().enumerate() {
            output.push_str(&format!(
                "[違反 {}] 制約{}: {}\n",
                i + 1,
                violation.constraint_id,
                violation.title
            ));
            output.push_str(&format!("重大度: {}\n", violation.severity));

            output.push_str("検出箇所:\n");
            for snippet in &violation.evidence.snippets {
                output.push_str(&format!("  - 「{}」\n", snippet));
            }

            if let Some(line) = violation.evidence.line {
                output.push_str(&format!("行番号: {}\n", line + 1));
            }

            if let Some(context) = &violation.evidence.context {
                output.push_str(&format!("コンテキスト: {}\n", context));
            }

            output.push_str("修正提案:\n");
            for suggestion in &violation.suggestions {
                output.push_str(&format!("  → {}\n", suggestion));
            }

            output.push('\n');
        }

        output
    }
}

impl Default for Report {
    fn default() -> Self {
        Self::new(vec![])
    }
}
