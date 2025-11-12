# sv-mint

## 目次
- [概要](#概要)
- [対象環境](#対象環境)
- [ビルド](#ビルド)
- [使用方法](#使用方法)
- [終了コード](#終了コード)
- [設定ファイル例](#設定ファイル例)
- [設定ロードの流れ](#設定ロードの流れ)
- [処理フロー](#処理フロー)
- [ステージ別 payload](#ステージ別-payload)
- [プラグイン仕様](#プラグイン仕様)
- [ログとサイズガード](#ログとサイズガード)
- [サンプルフィクスチャと再現コマンド](#サンプルフィクスチャと再現コマンド)
- [バイトコード抑止](#バイトコード抑止)
- [診断出力形式](#診断出力形式)
- [生成情報](#生成情報)
- [サードパーティーライセンス](#サードパーティーライセンス)

## 概要
sv-mint は SystemVerilog 向けの Linter です。Rust 製コアが sv-parser による前処理・構文解析を行い、raw_text / pp_text / cst / ast の各ステージ payload を構築します。複数入力ファイルを与えるとステージ処理はワーカースレッドを並列起動して自動的に分散されます。ステージ診断は常駐型の Python ホスト（`plugins/lib/rule_host.py`）へ NDJSON で送信され、ホストが ruleset に列挙された Python スクリプトの `check(req)` を順次実行します。CST はインライン JSON で配信し、payload サイズが警告・スキップの上限を超えた場合はステージをスキップして診断を代わりに返します。

## 対象環境
- OS: Windows 10 以降 / Linux / macOS
- Rust: stable（MSVC / GNU いずれも可）
- Python: 3.x（`python3` もしくは PATH 上の CPython）
- 文字コード: UTF-8（BOM 可）
- 改行コード: CRLF/LF 両対応（内部で LF 正規化）

## ビルド
```
rustup default stable
cargo build --release
```
生成物は `target\release\sv-mint.exe` に出力します。

## 使用方法
```
sv-mint --config .\sv-mint.toml path\to\file.sv
```
設定未指定時はカレントの `sv-mint.toml` を参照します。

## 終了コード
- 0: 診断なし
- 2: 診断あり（warning または error）
- 3: 入力不正、設定エラー、プラグイン異常、タイムアウト

## 設定ファイル例
```
[defaults]
timeout_ms_per_file = 3000

[plugin]
cmd = "python3"
args = ["-u","-B"]

[ruleset]
scripts = [
  "plugins/seq_blocking_in_alwaysff.py",
  "plugins/comb_nb_in_alwayscomb.py",
  "plugins/decl_unused_param.py",
  "plugins/decl_unused_net.py",
  "plugins/decl_unused_var.py"
]

[stages]
enabled = ["raw_text","pp_text","cst","ast"]

[svparser]
include_paths = []
defines = []
strip_comments = true
ignore_include = false
allow_incomplete = true

[logging]
level = "info"
format = "text" # text / json
stderr_snippet_bytes = 2048
show_stage_events = true
show_plugin_events = true
show_parse_events = true
```
`logging` セクションに追加された `format` は `text` / `json` を切り替えられます。それ以外の未知キーを設定した場合は起動時に警告ログで通知されます。
サイズガードのしきい値は現在固定です（警告 12,000,000 バイト、スキップ 16,000,000 バイト）。TOML での変更は未対応です。

## 設定ロードの流れ
1. `--config` でパスが指定されていればそのファイルを、未指定ならカレントディレクトリの `sv-mint.toml` を探索します。
2. UTF-8 で TOML を読み出し `Config` にデシリアライズします。
3. `validate_config` がプラグインコマンド、タイムアウト値、ステージ構成を検証します。
4. `logging` セクションを渡して `tracing` サブスクライバを初期化し、その後の `log_event` は `sv-mint::event` / `sv-mint::stage` へ出力されます。

## 処理フロー
1. 入力ソースを UTF-8 に正規化します。
2. sv-parser により前処理と構文解析を行います。
3. raw_text、pp_text、cst、ast の順にステージ payload を構築します。
4. ruleset.scripts に列挙した Python スクリプトを常駐ホストへロードし、ステージごとに `check(req)` を実行します（Rust 側は非同期 I/O でホストと通信します）。
5. 返却された違反オブジェクトの配列を集約して標準出力へ整形します。

## ステージ別 payload

Rust 側では全ステージを `StagePayload` enum で表現し、シリアライズ前にサイズガードを適用します。プラグインは下表のフィールドを参照できます。

### StagePayload 一覧

| stage      | payload 概要                                                                 | 備考 |
|------------|------------------------------------------------------------------------------|------|
| `raw_text` | `{ "text": "<LF 正規化済みソース>" }`                                        | 元ファイルを BOM 除去・LF 変換後に格納 |
| `pp_text`  | `{ "text": "<前処理済みソース>", "defines": [{"name","value"}...] }`         | `value` は `None` になる場合あり |
| `cst`      | `{ "mode": "inline", "cst_ir": {...} }` または `{ "mode": "none", "has_cst": bool }` | `cst_ir` には `line_starts` と `pp_text` が含まれる |
| `ast`      | `AstSummary` (`schema_version`, `decls`, `refs`, `symbols`, `assigns`, `scopes`, `pp_text`) | `pp_text` は Option で保持 |

### raw_text
```
{ "text": "<正規化済みソース>" }
```

### pp_text
```
{ "text": "<前処理済みソース>", "defines": [{ "name": "...", "value": "..." }] }
```

### cst
```
{
  "mode": "inline",
  "cst_ir": {
    "schema": 1,
    "format": "json",
    "sv_parser": "0.13.4",
    "file": "<入力ファイルパス>",
    "hash": "sha256:...",
    "line_starts": [0, ...],
    "include": { "text": true, "tokens": true },
    "pp_text": "<前処理済みソース>",
    "kind_table": [],
    "tok_kind_table": [],
    "tokens": [],
    "nodes": []
  }
}
```
`nodes` と `tokens` には `sv-parser` の構文木から抽出した情報が格納されます。`kind_table` / `tok_kind_table` はノード種別・トークン種別の名前テーブルで、`nodes[].first_token` と `last_token` を組み合わせて個々の構文片に含まれるトークン範囲を求められます。

### ast
```
{
  "schema_version": 1,
  "decls": [...],
  "refs": [...],
  "symbols": [...],
  "assigns": [...],
  "scopes": [],
  "pp_text": "<前処理済みソース>"
}
```

## プラグイン仕様

### 常駐ホストとのプロトコル
1. Rust 側が `plugins/lib/rule_host.py` を 1 回だけ起動し、最初の行で `{"kind":"init","scripts":[...絶対/相対パス...]}` を送信します。
2. ホストは各スクリプトを import し、`{"type":"ready"}` を返します。
3. ステージを処理するたびに `{"kind":"run_stage","stage":"ast", "path":"...","payload":{...}}` を送信します。
4. ホストは読み込んだ順に `module.check(req)` を呼び、返却された違反配列を結合して `{"type":"violations","violations":[...]}` を返します。
5. Rust 側終了時に `{"kind":"shutdown"}` を送信し、ホストは自身を終了させます。

Rust 側ホストクライアントは `tokio` ランタイム上で起動し、stdin/stdout/stderr を非同期ストリームとして扱います。`defaults.timeout_ms_per_file` はステージ単位のタイムアウトとして使われ、期限超過時はプロセスを kill した上で `PluginTimeout` イベントを記録します。`logging.stderr_snippet_bytes` に応じて stderr の末尾だけが `sv-mint::event` に添付され、大量出力でもメモリが暴走しません。

### スクリプトの書式
- ファイルは `check(req: dict) -> list[dict]` を定義してください。
- `req["stage"]`, `req["path"]`, `req["payload"]` を参照できます。
- 返り値は `Violation` 互換の辞書（下記フォーマット）をリストで返します。
- エラーを送出するとホストは `{ "type":"error", "detail":... }` を Rust 側へ返し、その時点で処理が停止します。

### Violation 辞書
```
{
  "rule_id": "<規則ID>",
  "severity": "error|warning|info",
  "message": "<メッセージ>",
  "location": { "line": n, "col": n, "end_line": n, "end_col": n }
}
```
未指定の場合は `severity_override` による上書きのみ行われます。

## サンプルプラグイン
```
def check(req):
    stage = req.get("stage")
    payload = req.get("payload") or {}
    count = len(payload.get("symbols") or [])
    return [{
        "rule_id": "debug.ping",
        "severity": "warning",
        "message": f"ping: stage={stage}, symbols={count}",
        "location": {"line":1,"col":1,"end_line":1,"end_col":1}
    }]
```

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
