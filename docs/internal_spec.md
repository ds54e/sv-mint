# 内部仕様書

## 0. 想定読者
sv-mint の開発者、設計管理者、あるいは将来的にコアを拡張したいエンジニアを対象とします。Rust コードベースのモジュール構成、データ契約、エラー分類、拡張時の留意点をまとめています。

## 1. モジュール構成
| ディレクトリ | 役割 |
| --- | --- |
| `src/lib.rs` | エントリーポイント。`core/`, `sv/`, `io/`, `plugin/` などの公開モジュールを束ねる。 |
| `src/bin/sv-mint.rs` | CLI (`clap`) 定義と `Pipeline` 起動。終了コードの割り当てを担当。 |
| `src/core/` | パイプライン制御、payload 構築、サイズガード、診断型など。 |
| `src/io/` | 設定ロード、テキストユーティリティ、CLI 出力整形。 |
| `src/sv/` | `sv-parser` 連携、CST/AST 構築、ラインマップ管理。 |
| `src/plugin/` | Python ホストとの IPC クライアント（tokio runtime）。 |
| `plugins/` | ルール実装と `rule_host.py`。 |
| `fixtures/` / `tests/` | CLI 向け統合テストと SystemVerilog サンプル。 |

## 2. パイプライン詳細
```
Pipeline::run_files -> run_file_batch/run_files_parallel -> run_file_with_host
```
1. `io::config::read_input` が UTF-8 読込 + BOM 除去 + LF 正規化を行い、元テキストと正規化済みテキストを返す。
2. `sv::driver::SvDriver` が `sv-parser` で `ParseArtifacts` を生成。raw_text/pp_text/CST/AST/defines を束ねる。include 先の LineMap は `sv::source::SourceCache` で管理し、Violation へ正しい `Location.file` を埋め込む。
3. `core::payload::StagePayload` がステージごとの JSON を構築。シリアライズ前に `core::size_guard::enforce_request_size` が 12/16 MB の閾値をチェック。
4. `plugin::client::PythonHost` が `tokio` ランタイム内で `python3` プロセスを起動し、`rule_host.py` と NDJSON で通信。タイムアウト時は `start_kill` でプロセスを停止し、`PluginTimeout` イベントを送出。
5. プラグイン応答は `Violation` ベクタとして集約され、CLI へ整形表示。`logging.stderr_snippet_bytes` を超える stderr は末尾のみがログに記録される。

## 3. データ契約
### 3.1 `StagePayload`
- `raw_text`: `InputText.normalized` をそのまま格納。
- `pp_text`: 前処理済みテキスト + `DefineInfo` の一覧。
- `cst`: `cst_ir`（`CstIr` 構造体）か `has_cst` フラグ。`sv-parser` が CST を返さない場合に備えて `mode: "none"` を送信。
- `ast`: `AstSummary`。`decls`/`refs`/`symbols`/`assigns`/`ports`/`pp_text` などを含む。シリアライズ対象に `line_map` を含めない点に注意。

### 3.2 `Violation`
```rust
pub struct Location {
    pub line: u32,
    pub col: u32,
    pub end_line: u32,
    pub end_col: u32,
    pub file: Option<String>,
}
```
CLI 出力は `location.file.unwrap_or(input_path)` を使用する。include ファイル対応のため、`SourceCache` が `file` を適切に設定することが重要。

### 3.3 エラー種別
| 型 | 用途 |
| --- | --- |
| `ConfigError` | 設定ファイルの読込・値検証・IO 失敗 |
| `ParseError` | sv-parser 前処理/解析/CST 取得失敗 |
| `PluginError` | Python ホスト起動・IO・JSON・タイムアウト |
| `OutputError` | CLI 表示用ファイル読み込み（主に `io::output` テスト用） |

## 4. ロギングとイベント
- `diag::logging::init` が `LoggingConfig` の `format/level` に従って `tracing_subscriber` を構築。`extra` に未知キーがある場合は warn。
- イベントは `Event` enum で型安全に管理され、`Ev` を通じて `log_event` へ渡す。`show_*_events` でカテゴリ別に抑制可能。
- stderr スニペット (`PluginStderr`) は `logging.stderr_snippet_bytes` が 0 の場合は無効化される。

## 5. 並列実行
- `Pipeline::run_files_parallel` は `std::thread::scope` + `available_parallelism` を用いてワーカースレッドを生成。各ワーカーが独自に `PythonHost` を持つため、プラグイン側は並列実行を想定して再入可能にしておく必要がある。
- ファイル数 < 論理 CPU の場合は入力数に合わせてスレッド数を縮小。

## 6. サイズガードとレスポンス制限
- 定数 `MAX_REQ_BYTES = 16_000_000`、`WARN_REQ_BYTES = 12_000_000`。required stage (`raw_text`, `pp_text`) で上限超過した場合は `Severity::Error` の `sys.stage.skipped.size` を返し、処理を打ち切る。
- レスポンス (`enforce_response_size`) も 16 MB 上限を持つ。超過時は `sys.stage.output.too_large` でステージ失敗。

## 7. エラー伝播と診断
- `sv::driver::parse_text` で parse 失敗した場合、`sys.parse.failed` を生成しつつ CLI へ即時出力。`summary.had_error` を立てて終了コード 3 を返す。
- プラグイン例外は `PluginError::ProtocolError` として報告され、ログにも `PluginExitNonzero` イベントが出力される。

## 8. 拡張指針
- ステージ追加を検討する場合は `types::Stage` と `StagePayload`、Toml の `stages.enabled` すべてに変更を施す必要がある。
- ルール作者に新規 payload を公開する際は、`docs/plugin_author.md` と `docs/rule_reference.md` を同時に更新して情報の一貫性を保つ。
- サイズガードのしきい値を可変にする際は TOML での検証ロジックと `SizePolicy` 生成処理をセットで変更する。

## 9. テスト戦略
- `tests/cli_smoke.rs` は主要ルールの E2E 確認。新規ルールを追加したら関連フィクスチャを作り、ここへ追記する。
- Rust 単体テストは未整備のため、機能追加時は必要に応じて `#[cfg(test)]` などで補完する。
- Python ルールは独立した `pytest` などでテストするか、CLI テストのフィクスチャに組み込む。

## 10. 将来拡張候補
- サイズガード閾値の設定化
- パイプライン結果サマリの JSON 出力
- Python ホストを gRPC 等へ置き換える検討

上記を踏まえて改修を行う際は、README および docs 配下のガイドを合わせて更新し、利用者と開発者双方のドキュメント整合性を保ってください。
