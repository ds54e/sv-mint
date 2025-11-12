# sv-mint

## 概要
sv-mint は SystemVerilog 向けの Linter です。Rust 製コアが sv-parser により前処理と構文解析を行い、各ステージの成果物を Python 製プラグインへ渡して診断を実行します。現在の実装では CST を要約せず、ステージ payload に インライン JSON で渡します。全ステージに共通のサイズガードを設け、直列化後のリクエストがしきい値を超えた場合はステージをスキップして警告を返します。

## 対象環境
- OS: Windows 10 以降
- Rust: stable (MSVC)
- Python: 3.x（py ランチャー経由で実行）
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
cmd = "py"
args = ["-3","-u","-B"]

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
stderr_snippet_bytes = 2048
show_stage_events = true
show_plugin_events = true
show_parse_events = true
```
サイズガードのしきい値は現在固定です（警告 12,000,000 バイト、スキップ 16,000,000 バイト）。TOML での変更は未対応です。

## 処理フロー
1. 入力ソースを UTF-8 に正規化します。
2. sv-parser により前処理と構文解析を行います。
3. raw_text、pp_text、cst、ast の順にステージ payload を構築します。
4. ruleset.scripts に列挙した Python プラグインをステージごとに順次起動します。
5. 各プラグインの JSON 応答を集約して出力します。

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

### 入力 (STDIN)
```
{
  "stage": "raw_text|pp_text|cst|ast",
  "path": "<ファイルパス>",
  "payload": { ... }
}
```
`type` フィールドは必須ではありません。

### 出力 (STDOUT)
```
{
  "type": "ViolationsStage",
  "stage": "<同一ステージ名>",
  "violations": [
    {
      "rule_id": "<規則ID>",
      "severity": "error|warning|info",
      "message": "<メッセージ>",
      "location": { "line": n, "col": n, "end_line": n, "end_col": n }
    }
  ]
}
```
STDERR はログとして記録します。

## サンプルプラグイン
```
import sys, json
req = json.loads(sys.stdin.read() or "{}")
stage = str(req.get("stage", ""))
viol = {
  "rule_id": "debug.ping",
  "severity": "warning",
  "message": "ping",
  "location": {"line": 1, "col": 1, "end_line": 1, "end_col": 1}
}
out = {"type": "ViolationsStage", "stage": stage, "violations": [viol]}
sys.stdout.write(json.dumps(out))
```

## サイズガード挙動
- 直列化後のリクエストが 12,000,000 バイト以上 16,000,000 バイト以下の場合に警告ログを出します。
- 直列化後のリクエストが 16,000,000 バイトを超える場合、そのステージを実行せず `sys.stage.skipped.size` を1件出力します。
- raw_text と pp_text は必須ステージとして扱い、スキップ時はエラー終了にします。

## バイトコード抑止
- 起動引数に `-B` を付与します。
- 必要に応じて環境変数 `PYTHONDONTWRITEBYTECODE=1` を設定します。
- `.gitignore` に `__pycache__/` と `*.pyc` を追加します。

## 診断出力形式
```
<path>:<line>:<col>: [<severity>] <rule_id>: <message>
```

## 生成情報
本ソフトウェアおよび本ドキュメントは ChatGPT により作成されています。

## サードパーティーライセンス
本ツールは MIT または Apache-2.0 ライセンスの下で配布される Rust クレート群を使用しています。
詳細は Cargo.toml を参照してください。
