//! エラー型の定義

use thiserror::Error;

/// 整合性チェックのエラー
#[derive(Error, Debug)]
pub enum IntegrityError {
    /// 入力が空
    #[error("入力が空です")]
    EmptyInput,

    /// 入力が大きすぎる
    #[error("入力が大きすぎます（最大: {max_bytes} バイト、実際: {actual_bytes} バイト）")]
    InputTooLarge {
        max_bytes: usize,
        actual_bytes: usize,
    },

    /// 無効なフォーマット指定
    #[error("無効な出力フォーマット: {0}")]
    InvalidFormat(String),

    /// ファイル読み込みエラー
    #[error("ファイル読み込みエラー: {0}")]
    FileRead(#[from] std::io::Error),

    /// JSON シリアライズエラー
    #[error("JSONシリアライズエラー: {0}")]
    JsonSerialize(#[from] serde_json::Error),

    /// その他のエラー
    #[error("{0}")]
    Other(String),
}
