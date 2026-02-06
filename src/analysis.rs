use std::collections::HashMap;

use crate::model::{Block, BlockKind, Document};

#[derive(Debug, Clone)]
pub struct Violation {
    pub constraint_id: u8,
    pub summary: String,
    pub details: String,
    pub suggestion: Option<String>,
    pub block_index: Option<usize>,
    pub assumption: bool,
}

#[derive(Debug, Clone)]
pub struct AnalysisReport {
    pub violations: Vec<Violation>,
    pub assumptions: Vec<String>,
}

impl AnalysisReport {
    pub fn new(assumptions: Vec<String>) -> Self {
        Self {
            violations: Vec::new(),
            assumptions,
        }
    }

    pub fn hard_fail(&self) -> bool {
        !self.violations.is_empty()
    }
}

pub fn analyze_document(doc: &Document) -> AnalysisReport {
    let config = AnalysisConfig::v0();
    let mut report = AnalysisReport::new(
        config
            .assumptions
            .iter()
            .map(|item| item.to_string())
            .collect(),
    );

    let normalized_blocks: Vec<NormalizedBlock> = doc
        .blocks
        .iter()
        .map(|block| NormalizedBlock::new(block))
        .collect();

    detect_preselect(&normalized_blocks, &config, &mut report);
    detect_visual_gap(&normalized_blocks, &config, &mut report);
    detect_cancel_hardness(&normalized_blocks, &config, &mut report);
    detect_hidden_disadvantages(&normalized_blocks, &config, &mut report);
    detect_mixed_meaning(&normalized_blocks, &config, &mut report);
    detect_unsubstantiated_comparison(&normalized_blocks, &config, &mut report);
    detect_unsubstantiated_urgency(&normalized_blocks, &config, &mut report);

    report
        .violations
        .sort_by_key(|violation| (violation.constraint_id, violation.block_index));
    report
}

#[derive(Clone)]
struct NormalizedBlock<'a> {
    block: &'a Block,
    text: String,
    raw: String,
}

impl<'a> NormalizedBlock<'a> {
    fn new(block: &'a Block) -> Self {
        Self {
            block,
            text: normalize(&block.text),
            raw: normalize(&block.raw),
        }
    }
}

struct AnalysisConfig {
    preselect_terms: Vec<&'static str>,
    action_terms: Vec<&'static str>,
    preference_terms: Vec<&'static str>,
    cancel_terms: Vec<&'static str>,
    cancel_difficulty_terms: Vec<&'static str>,
    adverse_terms: Vec<&'static str>,
    deferral_terms: Vec<&'static str>,
    fact_terms: Vec<&'static str>,
    claim_terms: Vec<&'static str>,
    cta_terms: Vec<&'static str>,
    comparison_terms: Vec<&'static str>,
    comparison_evidence_terms: Vec<&'static str>,
    urgency_terms: Vec<&'static str>,
    urgency_evidence_terms: Vec<&'static str>,
    attention_grab_terms: Vec<&'static str>,
    assumptions: Vec<&'static str>,
}

impl AnalysisConfig {
    fn v0() -> Self {
        Self {
            preselect_terms: vec![
                "デフォルト",
                "初期",
                "あらかじめ",
                "事前",
                "自動",
                "プリセット",
                "チェック済",
                "同意済",
                "登録済",
                "opt-out",
                "opt out",
                "preselect",
                "pre-selected",
                "pre selected",
                "checked by default",
                "prechecked",
                "[x]",
                "☑",
            ],
            action_terms: vec![
                "同意",
                "購読",
                "購入",
                "申込",
                "申し込み",
                "登録",
                "利用",
                "データ",
                "共有",
                "送信",
                "subscribe",
                "buy",
                "purchase",
                "sign up",
                "agree",
                "consent",
            ],
            preference_terms: vec!["おすすめ", "人気", "推奨", "ベスト", "一番", "no.1", "no1"],
            cancel_terms: vec![
                "解約",
                "退会",
                "停止",
                "キャンセル",
                "解除",
                "撤回",
                "中止",
                "refund",
                "cancel",
                "opt out",
            ],
            cancel_difficulty_terms: vec![
                "問い合わせ",
                "電話",
                "メール",
                "窓口",
                "別ページ",
                "後日",
                "手続き",
                "フォーム",
                "書面",
                "訪問",
                "本人確認",
            ],
            adverse_terms: vec!["料金", "価格", "制限", "例外", "条件", "追加費用", "違約金"],
            deferral_terms: vec![
                "別ページ",
                "別途",
                "折りたたみ",
                "詳細はこちら",
                "詳しくは",
                "規約",
                "faq",
                "リンク先",
                "後日",
            ],
            fact_terms: vec![
                "円",
                "¥",
                "%",
                "時間",
                "日",
                "期間",
                "料金",
                "価格",
                "条件",
                "制限",
                "提供",
                "含ま",
                "対応",
                "仕様",
                "機能",
                "対象",
                "開始日",
                "終了日",
                "在庫",
                "数量",
            ],
            claim_terms: vec![
                "おすすめ",
                "人気",
                "最高",
                "最強",
                "最適",
                "素晴らしい",
                "安心",
                "簡単",
                "お得",
                "便利",
                "no.1",
                "no1",
                "ベスト",
                "大人気",
                "圧倒的",
            ],
            cta_terms: vec![
                "申し込む",
                "申込",
                "登録",
                "購入",
                "同意",
                "今すぐ",
                "クリック",
                "開始",
                "参加",
                "無料で始める",
                "subscribe",
                "buy",
                "sign up",
                "download",
                "try",
                "get started",
                "apply",
            ],
            comparison_terms: vec![
                "no.1",
                "no1",
                "一番",
                "最安",
                "最速",
                "最高",
                "人気",
                "おすすめ",
                "ベスト",
                "ランキング",
                "トップ",
                "leading",
                "best",
                "top",
                "number one",
                "最も",
            ],
            comparison_evidence_terms: vec![
                "根拠",
                "出典",
                "調査",
                "アンケート",
                "データ",
                "※",
                "source",
                "n=",
                "対象",
                "期間",
                "比較対象",
                "条件",
                "時点",
                "http",
                "https",
            ],
            urgency_terms: vec![
                "今だけ",
                "限定",
                "残り",
                "わずか",
                "あと",
                "急げ",
                "本日限り",
                "先着",
                "タイムセール",
                "minutes left",
                "hours left",
                "ends soon",
                "締切",
            ],
            urgency_evidence_terms: vec![
                "期限",
                "在庫",
                "更新",
                "基準",
                "数量",
                "残数",
                "日時",
                "日付",
                "時点",
                "http",
                "https",
            ],
            attention_grab_terms: vec!["点滅", "blink", "カウントダウン", "countdown"],
            assumptions: vec![
                "v0 assumption: If a start/subscribe CTA exists but no stop/decline info appears, warn for constraint 3.",
                "v0 assumption: If a list of options contains preference words or emphasis markers, warn for constraint 2.",
            ],
        }
    }
}

fn detect_preselect(
    blocks: &[NormalizedBlock<'_>],
    config: &AnalysisConfig,
    report: &mut AnalysisReport,
) {
    for block in blocks {
        let has_preselect = contains_any(&block.text, &config.preselect_terms)
            || contains_any(&block.raw, &config.preselect_terms);
        let has_action = contains_any(&block.text, &config.action_terms);
        if has_preselect && has_action {
            report.violations.push(Violation {
                constraint_id: 1,
                summary: "Default selection is implied".to_string(),
                details: "Wording suggests preselected or auto-consent behavior.".to_string(),
                suggestion: Some(
                    "State that the initial state is unselected and only confirms after explicit action."
                        .to_string(),
                ),
                block_index: Some(block.block.index),
                assumption: false,
            });
        }
    }
}

fn detect_visual_gap(
    blocks: &[NormalizedBlock<'_>],
    config: &AnalysisConfig,
    report: &mut AnalysisReport,
) {
    let mut lists: HashMap<usize, Vec<&NormalizedBlock<'_>>> = HashMap::new();
    for block in blocks {
        if let BlockKind::ListItem { list_id, .. } = &block.block.kind {
            lists.entry(*list_id).or_default().push(block);
        }
    }

    for items in lists.values() {
        if items.len() < 2 {
            continue;
        }
        let mut flagged = false;
        let mut flagged_index = None;
        for item in items {
            let has_pref = contains_any(&item.text, &config.preference_terms);
            let has_emphasis = has_emphasis_marker(&item.block.raw);
            if has_pref || has_emphasis {
                flagged = true;
                flagged_index = Some(item.block.index);
                break;
            }
        }
        if flagged {
            report.violations.push(Violation {
                constraint_id: 2,
                summary: "Possible visual disparity between options".to_string(),
                details:
                    "Within the same list, preference words or emphasis markers are used.".to_string(),
                suggestion: Some(
                    "Keep equivalent options at the same emphasis level and remove highlight words."
                        .to_string(),
                ),
                block_index: flagged_index,
                assumption: true,
            });
        }
    }
}

fn detect_cancel_hardness(
    blocks: &[NormalizedBlock<'_>],
    config: &AnalysisConfig,
    report: &mut AnalysisReport,
) {
    let mut has_start_cta = false;
    let mut has_cancel = false;
    for block in blocks {
        if contains_any(&block.text, &config.cta_terms) {
            has_start_cta = true;
        }
        if contains_any(&block.text, &config.cancel_terms) {
            has_cancel = true;
            if contains_any(&block.text, &config.cancel_difficulty_terms) {
                report.violations.push(Violation {
                    constraint_id: 3,
                    summary: "Stopping action may be harder".to_string(),
                    details: "Cancellation/decline mentions extra steps or separate pages.".to_string(),
                    suggestion: Some(
                        "Ensure stopping is available on the same screen with no more steps than starting."
                            .to_string(),
                    ),
                    block_index: Some(block.block.index),
                    assumption: false,
                });
            }
        }
    }

    if has_start_cta && !has_cancel {
        report.violations.push(Violation {
            constraint_id: 3,
            summary: "Possible absence of stop/decline path".to_string(),
            details: "Start/subscribe CTA exists without any stop/decline info on the same screen."
                .to_string(),
            suggestion: Some("Describe how to stop/decline on the same screen.".to_string()),
            block_index: None,
            assumption: true,
        });
    }
}

fn detect_hidden_disadvantages(
    blocks: &[NormalizedBlock<'_>],
    config: &AnalysisConfig,
    report: &mut AnalysisReport,
) {
    for block in blocks {
        let has_adverse = contains_any(&block.text, &config.adverse_terms);
        let has_deferral = contains_any(&block.text, &config.deferral_terms);
        if has_adverse && has_deferral {
            report.violations.push(Violation {
                constraint_id: 4,
                summary: "Unfavorable info deferred or off-page".to_string(),
                details:
                    "Pricing/conditions appear to be deferred to another page or later.".to_string(),
                suggestion: Some(
                    "Place pricing, conditions, and limits alongside the main description."
                        .to_string(),
                ),
                block_index: Some(block.block.index),
                assumption: false,
            });
        }
    }
}

fn detect_mixed_meaning(
    blocks: &[NormalizedBlock<'_>],
    config: &AnalysisConfig,
    report: &mut AnalysisReport,
) {
    for block in blocks {
        let has_fact = contains_any(&block.text, &config.fact_terms) || has_number(&block.text);
        let has_claim = contains_any(&block.text, &config.claim_terms);
        let has_cta = contains_any(&block.text, &config.cta_terms);
        let kinds = [has_fact, has_claim, has_cta];
        let count = kinds.iter().filter(|flag| **flag).count();
        if count >= 2 {
            report.violations.push(Violation {
                constraint_id: 5,
                summary: "FACT/CLAIM/CTA mixed".to_string(),
                details: "Multiple meaning types appear in the same block.".to_string(),
                suggestion: Some(
                    "Separate facts, claims, and calls-to-action into distinct blocks."
                        .to_string(),
                ),
                block_index: Some(block.block.index),
                assumption: false,
            });
        }
    }
}

fn detect_unsubstantiated_comparison(
    blocks: &[NormalizedBlock<'_>],
    config: &AnalysisConfig,
    report: &mut AnalysisReport,
) {
    for block in blocks {
        if contains_any(&block.text, &config.comparison_terms) {
            let has_evidence = contains_any(&block.text, &config.comparison_evidence_terms);
            if !has_evidence {
                report.violations.push(Violation {
                    constraint_id: 6,
                    summary: "Comparison without evidence".to_string(),
                    details:
                        "Comparative wording appears without target, conditions, or evidence."
                            .to_string(),
                    suggestion: Some(
                        "Add comparison target, conditions, time, and evidence, or remove the comparison."
                            .to_string(),
                    ),
                    block_index: Some(block.block.index),
                    assumption: false,
                });
            }
        }
    }
}

fn detect_unsubstantiated_urgency(
    blocks: &[NormalizedBlock<'_>],
    config: &AnalysisConfig,
    report: &mut AnalysisReport,
) {
    for block in blocks {
        if contains_any(&block.text, &config.attention_grab_terms) {
            report.violations.push(Violation {
                constraint_id: 7,
                summary: "Attention-grabbing effect".to_string(),
                details: "Wording suggests blinking/countdown-style attention capture.".to_string(),
                suggestion: Some(
                    "Remove attention-grabbing effects and keep a factual tone.".to_string(),
                ),
                block_index: Some(block.block.index),
                assumption: false,
            });
        }

        if contains_any(&block.text, &config.urgency_terms) {
            let has_evidence = contains_any(&block.text, &config.urgency_evidence_terms)
                || has_numeric_time_hint(&block.text);
            if !has_evidence {
                report.violations.push(Violation {
                    constraint_id: 7,
                    summary: "Urgency/scarcity without evidence".to_string(),
                    details: "Urgency/scarcity wording appears without deadline or stock basis."
                        .to_string(),
                    suggestion: Some(
                        "State deadline/stock conditions and update basis, or remove urgency wording."
                            .to_string(),
                    ),
                    block_index: Some(block.block.index),
                    assumption: false,
                });
            }
        }
    }
}

fn contains_any(text: &str, terms: &[&str]) -> bool {
    terms.iter().any(|term| text.contains(term))
}

fn has_emphasis_marker(raw: &str) -> bool {
    raw.contains("**") || raw.contains("__") || raw.contains("!!") || raw.contains("！！")
}

fn has_number(text: &str) -> bool {
    text.chars()
        .any(|ch| ch.is_ascii_digit() || ('０'..='９').contains(&ch))
}

fn has_numeric_time_hint(text: &str) -> bool {
    if !has_number(text) {
        return false;
    }
    let units = ["年", "月", "日", "時間", "分", "秒", "時"];
    contains_any(text, &units)
}

fn normalize(text: &str) -> String {
    text.to_lowercase()
}
