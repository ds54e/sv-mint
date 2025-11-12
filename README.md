# sv-mint

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

## 処理フロー
1. 入力ソースを UTF-8 に正規化します。
2. sv-parser により前処理と構文解析を行います。
3. raw_text、pp_text、cst、ast の順にステージ payload を構築します。
4. ruleset.scripts に列挙した Python スクリプトを常駐ホストへロードし、ステージごとに `check(req)` を実行します（Rust 側は非同期 I/O でホストと通信します）。
5. 返却された違反オブジェクトの配列を集約して標準出力へ整形します。

## ステージ別 payload

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
現行実装では `nodes` と `tokens` が空の場合があります。プラグインは `pp_text` と `line_starts` を用いたフォールバックで動作させてください。将来、`nodes` と `tokens` が埋まる実装に置き換え予定です。

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

## サイズガード挙動
- 直列化後のリクエストが 12,000,000 バイト以上 16,000,000 バイト以下の場合に警告ログを出します。
- 直列化後のリクエストが 16,000,000 バイトを超える場合、そのステージを実行せず `sys.stage.skipped.size` を1件出力します。
- raw_text と pp_text は必須ステージとして扱い、スキップ時はエラー終了にします。
すべてのステージ結果は `StageOutcome` として集計され、path / stage / 所要時間が `sv-mint::stage` ログに出力されます。並列処理中でも各ステージの成功・スキップ状態をロギングで追跡できます。

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
