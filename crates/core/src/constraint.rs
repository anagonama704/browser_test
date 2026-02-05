//! 制約の定義

use serde::{Deserialize, Serialize};

/// 制約ID（1〜7）
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ConstraintId(pub u8);

impl ConstraintId {
    /// プリセレクト禁止
    pub const PRESELECT: Self = Self(1);
    /// 視覚格差禁止
    pub const VISUAL_BIAS: Self = Self(2);
    /// やめにくさ禁止
    pub const HARD_TO_CANCEL: Self = Self(3);
    /// 不利情報隠し禁止
    pub const HIDDEN_DISADVANTAGE: Self = Self(4);
    /// 事実と意見の混在禁止
    pub const FACT_OPINION_MIX: Self = Self(5);
    /// 根拠なき比較禁止
    pub const UNGROUNDED_COMPARISON: Self = Self(6);
    /// 急がせ希少性禁止
    pub const URGENCY_SCARCITY: Self = Self(7);

    /// 全ての制約IDを取得
    pub fn all() -> &'static [Self] {
        &[
            Self::PRESELECT,
            Self::VISUAL_BIAS,
            Self::HARD_TO_CANCEL,
            Self::HIDDEN_DISADVANTAGE,
            Self::FACT_OPINION_MIX,
            Self::UNGROUNDED_COMPARISON,
            Self::URGENCY_SCARCITY,
        ]
    }

    /// 制約のタイトルを取得
    pub fn title(self) -> &'static str {
        match self.0 {
            1 => "プリセレクトの禁止",
            2 => "視覚格差の禁止",
            3 => "やめにくさの禁止",
            4 => "不利情報隠しの禁止",
            5 => "事実と意見の混在禁止",
            6 => "根拠のない比較・優位性の禁止",
            7 => "根拠のない急がせ・希少性の禁止",
            _ => "不明な制約",
        }
    }

    /// 制約の説明を取得
    pub fn description(self) -> &'static str {
        match self.0 {
            1 => "ユーザーの明示的な意思なく、オプションが事前に選択されている状態を禁止する",
            2 => "選択肢間で視覚的な差異（サイズ、色、配置）を設けて特定の選択を誘導することを禁止する",
            3 => "解約・退会・キャンセル等の手続きを、開始手続きより困難にすることを禁止する",
            4 => "ユーザーにとって不利な情報を、目立たない場所に配置したり、アクセスしにくくすることを禁止する",
            5 => "客観的事実（数値、データ）と主観的意見（おすすめ、評価）を同一段落で混在させ、区別を困難にすることを禁止する",
            6 => "調査期間、母数、出典を明示せずに、No.1や最安等の優位性を主張することを禁止する",
            7 => "実際の期限や在庫状況に基づかない緊急性・希少性の演出を禁止する",
            _ => "不明",
        }
    }

    /// 修正提案を取得
    pub fn suggestion(self) -> &'static str {
        match self.0 {
            1 => "全てのオプションを未選択状態にし、ユーザーに明示的な選択を求める",
            2 => "全ての選択肢を同等のサイズ・色・配置で表示する",
            3 => "解約・退会手続きを、開始手続きと同等以上に簡単にする",
            4 => "不利な情報を、関連する有利な情報と同じ場所に同等の目立ち方で表示する",
            5 => "事実と意見を別のセクション・段落に分離し、明確にラベル付けする",
            6 => "比較・優位性の主張には、調査機関名、調査期間、母数（サンプル数）、出典を明記する",
            7 => "期限・在庫が事実なら具体的な条件と更新基準を明示する。根拠が無い場合は表現を撤去する",
            _ => "制約を確認して適切に修正する",
        }
    }
}

impl std::fmt::Display for ConstraintId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// 違反の重大度
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Severity {
    /// ハード制約違反（即不合格）
    HardFail,
    /// 警告（v0では未使用、将来拡張用）
    #[allow(dead_code)]
    Warning,
}

impl std::fmt::Display for Severity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Severity::HardFail => write!(f, "hard_fail"),
            Severity::Warning => write!(f, "warning"),
        }
    }
}

/// 制約の定義
#[derive(Debug, Clone)]
pub struct Constraint {
    pub id: ConstraintId,
    pub title: &'static str,
    pub description: &'static str,
    pub suggestion: &'static str,
}

impl Constraint {
    /// 制約IDから制約を作成
    pub fn from_id(id: ConstraintId) -> Self {
        Self {
            id,
            title: id.title(),
            description: id.description(),
            suggestion: id.suggestion(),
        }
    }
}
