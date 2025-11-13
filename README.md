# sv-mint

Rust 製の SystemVerilog lint パイプラインです。`sv-parser` が生成する raw_text / pp_text / cst / ast の各ステージ payload を Python プラグインへ渡し、並列実行・サイズガード・タイムアウトなどの運用機構を備えています。

## 目次
- [1. 概要](#1-概要)
- [2. ロール別クイックリンク](#2-ロール別クイックリンク)
- [3. クイックスタート](#3-クイックスタート)
- [4. 処理フロー](#4-処理フロー)
- [5. 開発・検証リソース](#5-開発検証リソース)
- [6. ログとサイズガード](#6-ログとサイズガード)
- [7. 出力と運用メモ](#7-出力と運用メモ)
- [8. 生成情報とライセンス](#8-生成情報とライセンス)

## 1. 概要
sv-mint は設計/検証チームが統一ルールで SystemVerilog を検証するための lint ツールです。

主な特徴:
- raw_text / pp_text / CST / AST を段階的にプラグインへ渡す多段パイプライン
- `ruleset.scripts` で列挙した Python ルールを常駐ホストが並列実行
- 16 MB の request/response サイズガードとステージ別タイムアウト
- `tracing` ベースの構造化/テキストログ、および stderr スニペット収集
- Windows / macOS / Linux で同一挙動（UTF-8、CRLF/LF 吸収）

## 2. ロール別クイックリンク

| 読者 | 知りたいこと | ドキュメント |
| --- | --- | --- |
| 初めてのユーザー | 実行方法、`sv-mint.toml` 設定、FAQ | [docs/user_guide.md](docs/user_guide.md) |
| ルール背景や仕様をまとめて調べたい人 | `rule_id` ごとのステージ・Severity・対処方法と検出条件 | [docs/plugins/](docs/plugins) |
| プラグインを追加・改造したい人 | payload 仕様、Violation 形式、デバッグ方法 | [docs/plugin_author.md](docs/plugin_author.md) |
| Rust コアを拡張したい人 | コア構造、データ契約、エラー分類 | [docs/internal_spec.md](docs/internal_spec.md) |

## 3. クイックスタート

### 3.1 必要環境
- OS: Windows 10 以降 / Linux / macOS
- Rust: stable toolchain
- Python: 3.x（`python3` が実行可能であること）
- 文字コード: UTF-8（BOM 可）、改行は LF/CRLF どちらでも可

### 3.2 ビルド
```bash
rustup default stable
cargo build --release
```
生成物は `target/release/sv-mint`（Windows は `.exe`）に配置されます。

### 3.3 実行
```bash
sv-mint --config ./sv-mint.toml path/to/file.sv
```
`--config` を省略するとカレントディレクトリの `sv-mint.toml` を読み込みます。

### 3.4 終了コード
| コード | 意味 |
| ------ | ---- |
| 0 | 診断なし |
| 2 | 違反あり（warning/error 含む） |
| 3 | 入力・設定・プラグイン・タイムアウト等の致命的エラー |

## 4. 処理フロー
1. 入力を UTF-8 に正規化し、`sv-parser` で前処理 (`pp_text`) と構文解析を実施。
2. raw_text / pp_text / CST / AST の `StagePayload` を生成し、JSON サイズを検査。
3. 常駐ホスト `plugins/lib/rule_host.py` へ NDJSON でリクエスト。`ruleset.scripts` の `check(req)` を順番に実行。
4. 返却された Violation を集約し、CLI 表示と `tracing` イベントへ同時出力。

payload 仕様やホストの詳細は [docs/plugin_author.md](docs/plugin_author.md) と [docs/internal_spec.md](docs/internal_spec.md) を参照してください。

## 5. 開発・検証リソース
- `docs/user_guide.md#sv-minttoml-設定`: `sv-mint.toml` のテンプレートとオプション解説。
- `docs/plugins/lang_construct_rules.md` など: CLI で出た `rule_id` の詳細や修正方針を確認するためのルール別ガイド。
- `fixtures/`: CLI テストやルール検証に使える SystemVerilog サンプル。
- `tests/cli_smoke.rs`: 代表的なルールを網羅する E2E テスト。
- `plugins/`: 標準ルール実装。`rule_host.py` のホットリロードや `debug_ping.py` などのテンプレートを含みます。

質問や改善案は Issue / Pull Request で歓迎しています。

## 6. ログとサイズガード

### 6.1 ロギング
- `logging.level` は `tracing` のフィルタに転送され、`sv-mint::event`（イベント）、`sv-mint::stage`（ステージ結果）、`sv-mint::logging`（設定警告）が発火します。
- `logging.format = "json"` を指定すると構造化ログを出力。未知オプションは警告ログで通知。
- `show_stage_events` や `show_plugin_stderr` を使ってデバッグ出力を制御できます。

### 6.2 サポート済みルール例
- 行長・ASCII 制約・タブ禁止・プリプロ整形などのテキストフォーマット
- モジュール/信号/ポート命名（`clk/rst` 順序、差動ペア、パイプライン `_q2`）、`typedef` `_e/_t`、`parameter` UpperCamelCase
- `always_comb` 推奨、`always_ff` の非同期リセット、`always_latch` / `always @*` 禁止、`case` の `default`・`unique` 推奨、複数ノンブロッキング代入検知
- `package` 内 `` `define`` や複数 `package` 宣言、`module` インスタンスの `.*` / 位置指定ポート、SPDX ヘッダー、グローバル `` `define`` などのガバナンス

### 6.3 今後の拡張アイデア
- 多ビット信号のブール使用禁止や幅推論などのビット幅解析
- `unique/priority case` の網羅率検証、`casez/casex`、`X` 伝播解析
- `package` 依存グラフの循環検知、`parameter/localparam` の高度な整合性チェック
- コメント整列や詳細なフォーマット細則など、より厳密なスタイルルール

## 7. 出力と運用メモ

### 7.1 バイトコード抑止
- CLI 実行時に `-B` を付与（既定 args を参照）。
- `PYTHONDONTWRITEBYTECODE=1` を設定。
- `.gitignore` に `__pycache__/` と `*.pyc` を登録済み。

### 7.2 診断出力形式
```
<path>:<line>:<col>: [<severity>] <rule_id>: <message>
```
違反が 1 件でも発生すると終了コード 2 を返します。複数ファイルをまとめて検証する場合は `cargo run -- fixtures/*.sv` のようにワイルドカードを渡してください。

## 8. 生成情報とライセンス
- 本ソフトウェアおよび本ドキュメントは ChatGPT により作成されています。
- 依存する Rust クレートは MIT または Apache-2.0 ライセンスです。詳細は Cargo.toml を参照してください。
