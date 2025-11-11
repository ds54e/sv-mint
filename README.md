# sv-mint

## 概要
sv-mint は SystemVerilog 用の Linter です。Rust 製のコアが sv-parser を用いて前処理と構文解析を行い、各ステージで得られたデータを Python 製プラグインに渡して診断を実行します。

## 対象環境
- 対応 OS: Windows 10 以降
- Rust: stable (MSVC)
- Python: 3.x（py ランチャー経由で実行）
- 文字コード: UTF-8（BOM 可）
- 改行コード: CRLF/LF 両対応（内部で LF 正規化）

## ビルド方法
```bash
rustup default stable
cargo build --release
```
生成物は `target\release\sv-mint.exe` に出力されます。

## 使用方法
```bash
sv-mint --config .\sv-mint.toml path\to\file.sv
```
設定ファイル未指定時はカレントディレクトリの `sv-mint.toml` を探索します。

### 終了コード仕様
| コード | 内容 |
|--------|------|
| 0 | 診断なし |
| 2 | 診断あり（warning または error） |
| 3 | 入力不正、設定エラー、プラグイン異常、タイムアウト |

## 設定ファイル例
```toml
[defaults]
timeout_ms_per_file = 3000

[plugin]
cmd = "py"
args = ["-3","-u"]

[ruleset]
scripts = [
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
show_stage_events = false
show_plugin_events = false
show_parse_events = false
```

## 処理フロー
1. 入力ファイルを UTF-8 正規化
2. sv-parser により前処理および構文解析
3. 各ステージ（raw_text, pp_text, cst, ast）を順に生成
4. ruleset.scripts に列挙された Python プラグインを順に起動
5. 各プラグインの JSON 応答を集約し、整形出力

## プラグイン仕様

### 入力 (STDIN)
```json
{
  "type": "CheckFileStage",
  "stage": "raw_text|pp_text|cst|ast",
  "path": "<ファイルパス>",
  "payload": { ... }
}
```

### 出力 (STDOUT)
```json
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
応答は 1 JSON オブジェクトのみ出力してください。STDERR はログとして捕捉されます。

## ステージ別 payload 構造

### raw_text
```json
{ "text": "<正規化済みソース>" }
```

### pp_text
```json
{ "text": "<前処理済みソース>" }
```

### cst
```json
{ "text": "<構文木解析用プレーンテキスト>" }
```

### ast
```json
{
  "decls": [ ... ],
  "refs": [ ... ]
}
```

## サンプルプラグイン
```python
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

## 診断出力形式
```
<path>:<line>:<col>: [<severity>] <rule_id>: <message>
```
正規表現例:  
`^(.+?):(\d+):(\d+): \[(error|warning|info)\] ([^:]+): (.+)$`

CI 連携では終了コード 2 を検出して品質ゲートに利用可能です。

## トラブルシューティング
- プラグイン出力が空または不正 JSON の場合、エラー終了します。
- タイムアウトは `defaults.timeout_ms_per_file` で設定します。
- Windows TOML で絶対パスを記述する際はバックスラッシュを二重にしてください。

## 生成情報
本ソフトウェアおよび本ドキュメントは ChatGPT により作成されています。

## サードパーティーライセンス
本ツールは MIT または Apache-2.0 ライセンスの下で配布される Rust クレート群を使用しています。
詳細は Cargo.toml を参照してください。