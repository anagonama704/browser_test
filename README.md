# Integrity Check - ハード制約チェッカー

**誤認誘導（Misleading Representation）の排除** を目的とした、思想主導Webブラウザ向けのハード制約チェックツールです。

## 守る思想

このツールは、ユーザーが以下のような状況に陥ることを防ぐために設計されています：

- 事実と異なる認識に基づいて意思決定する
- 重要な情報を見落として判断する
- 心理的圧力により不本意な選択をする

**提供者の意図（強調したい、やめにくくしたい、急がせたい等）は考慮しません。**
あくまでユーザーの正確な理解と自由な意思決定を支援します。

## ハード制約一覧

詳細は [constraints_v0_1.md](.cursor/rules/constraints_v0_1.md) を参照してください。

| ID | 名称 | 概要 |
|----|------|------|
| ① | プリセレクトの禁止 | ユーザーの明示的な意思なく、オプションが事前に選択されている状態を禁止 |
| ② | 視覚格差の禁止 | 選択肢間で視覚的な差異を設けて特定の選択を誘導することを禁止 |
| ③ | やめにくさの禁止 | 解約・退会・キャンセル等の手続きを、開始手続きより困難にすることを禁止 |
| ④ | 不利情報隠しの禁止 | ユーザーにとって不利な情報を、目立たない場所に配置することを禁止 |
| ⑤ | 事実と意見の混在禁止 | 客観的事実と主観的意見を同一段落で混在させることを禁止 |
| ⑥ | 根拠のない比較・優位性の禁止 | 調査期間、母数、出典を明示せずにNo.1等の優位性を主張することを禁止 |
| ⑦ | 根拠のない急がせ・希少性の禁止 | 実際の期限や在庫状況に基づかない緊急性・希少性の演出を禁止 |

**すべてのハード制約は「即不合格（Hard fail）」として処理されます。例外は認めません。**

## インストール

```bash
cargo build --release
```

ビルド後、以下のバイナリが生成されます：
- `target/release/integrity_check` - CLIツール
- `target/release/integrity_mcp` - MCPサーバ

## CLIの使い方

### 基本的な使い方

```bash
# ファイルを入力として使用
integrity_check --input ./sample.txt --format human

# 標準入力から読み込み
echo "顧客満足度No.1！今だけ特別価格！" | integrity_check --stdin --format human

# JSON形式で出力
integrity_check --input ./sample.txt --format json
```

### オプション

| オプション | 説明 |
|-----------|------|
| `-i, --input <PATH>` | 入力ファイルのパス |
| `--stdin` | 標準入力から読み込む |
| `-f, --format <FORMAT>` | 出力フォーマット（`human` または `json`）。デフォルト: `human` |
| `--max-size <BYTES>` | 最大入力サイズ（バイト）。デフォルト: 1048576 |

### 出力例（human形式）

```
⚠ 2 件の違反が検出されました
========================================

[違反 1] 制約6: 根拠のない比較・優位性の禁止
重大度: hard_fail
検出箇所:
  - 「No.1」
行番号: 1
コンテキスト: 顧客満足度No.1！今だけ特別価格！
修正提案:
  → 比較・優位性の主張には、調査機関名、調査期間、母数（サンプル数）、出典を明記する

[違反 2] 制約7: 根拠のない急がせ・希少性の禁止
重大度: hard_fail
検出箇所:
  - 「今だけ」
行番号: 1
コンテキスト: 顧客満足度No.1！今だけ特別価格！
修正提案:
  → 期限・在庫が事実なら具体的な条件と更新基準を明示する。根拠が無い場合は表現を撤去する
```

### 出力例（JSON形式）

```json
{
  "version": "0.1",
  "violations": [
    {
      "constraint_id": 6,
      "title": "根拠のない比較・優位性の禁止",
      "severity": "hard_fail",
      "evidence": {
        "snippets": ["No.1"],
        "line": 0,
        "context": "顧客満足度No.1！今だけ特別価格！"
      },
      "suggestions": [
        "比較・優位性の主張には、調査機関名、調査期間、母数（サンプル数）、出典を明記する"
      ]
    }
  ]
}
```

### 終了コード

- `0`: 違反なし
- `1`: 違反あり

## MCPサーバの使い方

### サーバの起動

```bash
integrity_mcp
```

MCPサーバはstdio経由でJSON-RPCメッセージを処理します。

### Cursorでの設定

Cursorの`mcp.json`に以下を追加してください：

```json
{
  "mcpServers": {
    "integrity": {
      "command": "/path/to/integrity_mcp",
      "args": [],
      "env": {
        "INTEGRITY_RULES_PATH": "/path/to/.cursor/rules"
      }
    }
  }
}
```

### 提供するResources

| URI | ファイル | 説明 |
|-----|---------|------|
| `integrity://project` | project_v0_1.md | プロジェクト仕様書 |
| `integrity://philosophy` | philosophy_v0_1.md | 設計哲学 |
| `integrity://constraints` | constraints_v0_1.md | ハード制約一覧 |

### 提供するTools

#### `check_constraints`

テキストのハード制約違反を検出します。

**入力スキーマ:**
```json
{
  "input": "チェック対象のテキスト",
  "format": "json"  // または "human"（オプション、デフォルト: "human"）
}
```

**出力:**
違反レポート（テキストまたはJSON）

## プロジェクト構成

```
/
├── .cursor/rules/
│   ├── project_v0_1.md       # プロジェクト仕様
│   ├── philosophy_v0_1.md    # 設計哲学
│   └── constraints_v0_1.md   # ハード制約一覧
├── crates/
│   ├── core/                 # 共通モデル（Violation, Evidence, Report）
│   ├── parser/               # テキスト解析（段落/文分割）
│   ├── rules/                # 制約①〜⑦のルール実装
│   ├── cli/                  # integrity_check CLI
│   └── mcp/                  # integrity_mcp MCPサーバ
├── examples/
│   ├── samples_bad.txt       # 違反サンプル
│   └── samples_good.txt      # 適正サンプル
├── Cargo.toml
└── README.md
```

## 開発

### ビルド

```bash
cargo build
```

### テスト

```bash
cargo test
```

### フォーマット

```bash
cargo fmt
```

### Lintチェック

```bash
cargo clippy
```

## v0の制限事項

- 入力はプレーンテキストのみ対応（将来的にBlueprint等へ拡張予定）
- 語彙シグナルベースの簡易検出（機械学習等の高度な検出は未実装）
- 日本語テキストに特化（英語等の多言語対応は限定的）

## ライセンス

MIT License

## 関連ドキュメント

- [プロジェクト仕様](.cursor/rules/project_v0_1.md)
- [設計哲学](.cursor/rules/philosophy_v0_1.md)
- [ハード制約一覧](.cursor/rules/constraints_v0_1.md)
