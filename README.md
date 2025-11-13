# sv-mint

SystemVerilog 向けの lint パイプラインです。Rust 製コアが `sv-parser` を用いて前処理・構文解析を行い、raw_text / pp_text / cst / ast の各ステージ payload を Python プラグインへ届けます。複数入力は自動的に並列化され、サイズガード・タイムアウト・stderr 抜粋などの運用ガードも標準装備です。

## 目次
- [概要](#概要)
- [クイックスタート](#クイックスタート)
- [ドキュメント案内](#ドキュメント案内)
- [処理フロー概要](#処理フロー概要)
- [サンプル設定](#サンプル設定)
- [サポートリソース](#サポートリソース)

## 概要
sv-mint は設計/検証チームが統一ルールで SystemVerilog を検証できるように設計されています。Rust コアは I/O や並列制御を受け持ち、ルール実装は Python プラグインに委ねるため、既存資産を流用しつつパフォーマンスと拡張性を両立できます。

特徴:
- sv-parser から得た raw_text / pp_text / CST / AST を段階的に Python へ配信
- `ruleset.scripts` に列挙したプラグインを常駐ホストでまとめて実行
- 16 MB を上限にした request/response サイズガードとステージごとのタイムアウト
- `tracing` ベースの JSON/TEXT ログ、stderr スニペット収集
- Windows / macOS / Linux で同一挙動（UTF-8 + CRLF/LF 自動吸収）

## クイックスタート

### 必要環境
- OS: Windows 10 以降 / Linux / macOS
- Rust: stable toolchain
- Python: 3.x（`python3` 実行可能であること）
- 文字コード: UTF-8（BOM 可）、改行は LF/CRLF いずれでも可

### ビルド
```bash
rustup default stable
cargo build --release
```
生成物は `target/release/sv-mint`（Windows は `.exe`）に配置されます。

### 走らせる
```bash
sv-mint --config ./sv-mint.toml path/to/file.sv
```
`--config` を省略するとカレントディレクトリの `sv-mint.toml` を読み込みます。

### 終了コード
| コード | 意味 |
| ------ | ---- |
| 0 | 診断なし |
| 2 | 違反あり（warning/error 含む） |
| 3 | 入力・設定・プラグイン・タイムアウト等の致命的エラー |

## ドキュメント案内

| ファイル | 対象読者 | 主な内容 |
| --- | --- | --- |
| [README.md](README.md) | すべて | ツール概要とリンク集 |
| [docs/user_guide.md](docs/user_guide.md) | 一般ユーザー | 実行方法、`sv-mint.toml` 設定、ログ解析、FAQ |
| [docs/rule_reference.md](docs/rule_reference.md) | ルール利用者 | ルール一覧、ID/ステージ/検出条件、典型メッセージ |
| [docs/rule_detail_user.md](docs/rule_detail_user.md) | 一般ユーザー | ルールごとの背景・Severity・修正方針をまとめた対処ガイド |
| [docs/plugin_author.md](docs/plugin_author.md) | プラグイン作者 | I/O 仕様、Violation 構造、デバッグ・テスト方法 |
| [docs/internal_spec.md](docs/internal_spec.md) | 開発者 | コア構造、データ契約、エラー分類、拡張指針 |

まず README で概要を掴み、ユーザーは user_guide / rule_reference へ。プラグイン開発やコア改修時は plugin_author / internal_spec を参照してください。

## 処理フロー概要
1. 入力を UTF-8 に正規化し、`sv-parser` で前処理 (`pp_text`) と構文解析を行う。
2. raw_text / pp_text / CST / AST の payload を `StagePayload` として生成、JSON サイズを検査。
3. 常駐ホスト `plugins/lib/rule_host.py` へ NDJSON でリクエスト。ホストは `ruleset.scripts` の `check(req)` を順番に実行し、Violation 配列を返す。
4. 取得した違反を集約し、CLI 表示と `tracing` イベントへ出力。

詳細な payload 仕様やプロトコルは [docs/plugin_author.md](docs/plugin_author.md) と [docs/internal_spec.md](docs/internal_spec.md) を参照してください。

## サンプル設定
代表的な `sv-mint.toml` の全体例とキーごとの説明は [docs/user_guide.md](docs/user_guide.md#sv-minttoml-設定) に集約しています。自組織向けのテンプレートを作成する際はそちらを参照して最新のオプションと注意事項を確認してください。

## サポートリソース
- `fixtures/`: CLI テストやルール検証に使える SystemVerilog サンプル。
- `tests/cli_smoke.rs`: 代表的なルールを網羅する E2E テスト。組み込みルールの挙動確認に利用できます。
- `plugins/`: 標準ルールの実装例。`rule_host.py` によるホットリロードや `debug_ping.py` などのテンプレートも含みます。

問い合わせや改善案は Issue / Pull Request で歓迎しています。

## ログとサイズガード

### ロギング
- `logging.level` は `tracing` のフィルタに転送され、`sv-mint::event`（イベント）、`sv-mint::stage`（ステージ結果）、`sv-mint::logging`（設定警告）の各ターゲットが発火します。
- `logging.format` に `json` を指定すると構造化 JSON ログを出力します。未知のキーは警告ログで通知されます。
- `show_stage_events` / `show_plugin_events` / `show_parse_events` はイベント種別ごとの出力可否を制御します。
- `stderr_snippet_bytes` はプラグイン stderr の末尾バイト数を制限し、超過分を自動的に切り詰めてログへ添付します。

### サイズガード挙動
- 直列化後のリクエストが 12,000,000 バイト以上 16,000,000 バイト以下の場合に警告ログを出します。
- 直列化後のリクエストが 16,000,000 バイトを超える場合、そのステージを実行せず `sys.stage.skipped.size` を1件出力します。
- raw_text と pp_text は必須ステージとして扱い、スキップ時はエラー終了にします。

すべてのステージ結果は `StageOutcome` として集計され、path / stage / 所要時間が `sv-mint::stage` ログに出力されます。並列処理中でも各ステージの成功・スキップ状態をロギングで追跡できます。

## サンプルフィクスチャと再現コマンド

`fixtures/` ディレクトリには代表的な規約違反を再現する SystemVerilog ソースを保管しています。ルール実装やリグレッション確認の際に以下のコマンドを実行して挙動を確認できます。

| フィクスチャ | 想定ルール | コマンド |
| --- | --- | --- |
| `fixtures/format_line_length_violation.sv` | `format.line_length`（行長 100 列超過） | `cargo run -- fixtures/format_line_length_violation.sv` |
| `fixtures/port_wildcard_violation.sv` | `module.no_port_wildcard`（`.*` 接続禁止） | `cargo run -- fixtures/port_wildcard_violation.sv` |
| `fixtures/case_missing_default.sv` | `case.missing_default`（default 項目必須） | `cargo run -- fixtures/case_missing_default.sv` |
| `fixtures/end_else_newline.sv` | `format.end_else_inline`（`end else` 同一行） | `cargo run -- fixtures/end_else_newline.sv` |
| `fixtures/if_without_begin.sv` | `format.begin_required`（複数行ブロックの begin/end 必須） | `cargo run -- fixtures/if_without_begin.sv` |
| `fixtures/whitespace_violations.sv` | `format.no_tabs` / `format.no_trailing_whitespace` | `cargo run -- fixtures/whitespace_violations.sv` |
| `fixtures/spacing_violations.sv` | `format.call_spacing` / `format.case_colon_spacing` など | `cargo run -- fixtures/spacing_violations.sv` |
| `fixtures/naming_violations.sv` | `naming.module_case` / `naming.clk_order` など | `cargo run -- fixtures/naming_violations.sv` |
| `fixtures/lang_violations.sv` | `lang.prefer_always_comb` / `lang.no_delays` | `cargo run -- fixtures/lang_violations.sv` |
| `fixtures/global_define_violations.sv` | `global.prefer_parameters` / `global.local_define_undef` | `cargo run -- fixtures/global_define_violations.sv` |
| `fixtures/multiple_nonblocking.sv` | `flow.multiple_nonblocking` | `cargo run -- fixtures/multiple_nonblocking.sv` |
| `fixtures/width_literal_violation.sv` | `width.unsized_base_literal` | `cargo run -- fixtures/width_literal_violation.sv` |
| `fixtures/case_unique_violation.sv` | `lang.case_requires_unique` | `cargo run -- fixtures/case_unique_violation.sv` |
| `fixtures/case_begin_violation.sv` | `format.case_begin_required` | `cargo run -- fixtures/case_begin_violation.sv` |
| `fixtures/package_mismatch.sv` | `package.multiple` / `package.define_in_package` | `cargo run -- fixtures/package_mismatch.sv` |
| `fixtures/module_inst_violation.sv` | `module.named_ports_required` / `module.no_port_wildcard` | `cargo run -- fixtures/module_inst_violation.sv` |
| `fixtures/header_missing.sv` | `header.missing_spdx` | `cargo run -- fixtures/header_missing.sv` |
| `fixtures/typedef_violation.sv` | `typedef.enum_suffix` / `typedef.type_suffix` | `cargo run -- fixtures/typedef_violation.sv` |
| `fixtures/indent_violation.sv` | `format.indent_multiple_of_two` / `format.line_continuation_right` | `cargo run -- fixtures/indent_violation.sv` |
| `fixtures/parameter_violation.sv` | `naming.parameter_upper` | `cargo run -- fixtures/parameter_violation.sv` |
| `fixtures/always_ff_violation.sv` | `lang.always_ff_reset` / `lang.always_comb_at` | `cargo run -- fixtures/always_ff_violation.sv` |

## LowRISC ルール実装状況

### 実装済み（中〜小規模）
- 文字種・行長・タブ/末尾空白・プリプロ指令・行継続・`case` の `begin/end` などの整形ルール
- モジュール/信号/ポート命名（`clk/rst` 順序、差動ペア、パイプライン段 `_q2`）、`typedef` `_e/_t`、`parameter` UpperCamelCaseなどの命名規則
- `always_comb` 推奨、`always_ff` の非同期リセット、`always_latch`・`always @*` 禁止、`case` の `default`・`unique/priority` 推奨、複数ノンブロッキング代入検知などの言語安全ルール
- `package` 内 `` `define`` や複数 `package` 宣言、`module` インスタンスの `.*` / 位置指定ポート、SPDX ヘッダー、グローバル `` `define`` など設定・ヘッダーポリシー
- CLI スモークテスト 21 ケースで代表的な違反を検証済み

### 未実装（大規模解析が必要）
- ブールでの多ビット使用禁止、暗黙幅ミスマッチ、演算結果の幅管理などのビット幅解析
- `unique/priority case` の網羅率や `casez/casex`、`X` 伝播の検証
- `package` 依存グラフの循環検知や `parameter/localparam` の詳細な運用ルール
- タブラーアライメントやコメント配置など、より厳密なフォーマット細則全般

これらの未実装項目は新しい解析パスや IR 追加が必要になるため、今後の大規模タスクとして別途対応予定です。

どのコマンドも違反が発生した場合は終了コード 2 で終了します。複数ファイルを一度に検証したい場合は `cargo run -- fixtures/*.sv` のようにワイルドカードを渡してください。

## バイトコード抑止
- 起動引数に `-B` を付与します（既定 args を参照）。
- 必要に応じて環境変数 `PYTHONDONTWRITEBYTECODE=1` を設定します。
- `.gitignore` に `__pycache__/` と `*.pyc` を追加済みです。

## 診断出力形式
```
<path>:<line>:<col>: [<severity>] <rule_id>: <message>
```

## 生成情報
本ソフトウェアおよび本ドキュメントは ChatGPT により作成されています。

## サードパーティーライセンス
本ツールは MIT または Apache-2.0 ライセンスの下で配布される Rust クレート群を使用しています。
詳細は Cargo.toml を参照してください。
