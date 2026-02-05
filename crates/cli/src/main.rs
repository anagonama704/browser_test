//! integrity_check CLI
//!
//! テキストのハード制約違反を検出するCLIツール

use anyhow::{Context, Result};
use clap::{Parser, ValueEnum};
use std::fs;
use std::io::{self, Read};

/// ハード制約チェッカー CLI
///
/// 入力テキストに対して制約①〜⑦の違反を検出し、レポートを出力します。
#[derive(Parser, Debug)]
#[command(name = "integrity_check")]
#[command(author, version, about, long_about = None)]
struct Args {
    /// 入力ファイルのパス
    #[arg(short, long, conflicts_with = "stdin")]
    input: Option<String>,

    /// 標準入力から読み込む
    #[arg(long)]
    stdin: bool,

    /// 出力フォーマット
    #[arg(short, long, value_enum, default_value = "human")]
    format: OutputFormat,

    /// 最大入力サイズ（バイト）
    #[arg(long, default_value = "1048576")]
    max_size: usize,
}

/// 出力フォーマット
#[derive(Debug, Clone, Copy, ValueEnum)]
enum OutputFormat {
    /// 人間が読みやすい形式
    Human,
    /// JSON形式
    Json,
}

fn main() -> Result<()> {
    let args = Args::parse();

    // 入力を取得
    let input = get_input(&args)?;

    // 入力の検証
    validate_input(&input, args.max_size)?;

    // チェック実行
    let report = integrity_rules::check_text(&input);

    // 出力
    let output = match args.format {
        OutputFormat::Human => report.to_human_readable(),
        OutputFormat::Json => report.to_json().context("JSON出力に失敗しました")?,
    };

    println!("{}", output);

    // 違反がある場合は終了コード1を返す
    if report.has_violations() {
        std::process::exit(1);
    }

    Ok(())
}

/// 入力を取得
fn get_input(args: &Args) -> Result<String> {
    if args.stdin {
        let mut input = String::new();
        io::stdin()
            .read_to_string(&mut input)
            .context("標準入力の読み込みに失敗しました")?;
        Ok(input)
    } else if let Some(path) = &args.input {
        fs::read_to_string(path)
            .with_context(|| format!("ファイルの読み込みに失敗しました: {}", path))
    } else {
        anyhow::bail!("入力を指定してください。--input または --stdin を使用してください。")
    }
}

/// 入力を検証
fn validate_input(input: &str, max_size: usize) -> Result<()> {
    if input.trim().is_empty() {
        anyhow::bail!("入力が空です");
    }

    if input.len() > max_size {
        anyhow::bail!(
            "入力が大きすぎます（最大: {} バイト、実際: {} バイト）",
            max_size,
            input.len()
        );
    }

    Ok(())
}
