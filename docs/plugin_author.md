# プラグイン作者ガイド

## 0. 想定読者
sv-mint の Python ルールを新規に作成・改修するエンジニアを対象とします。Rust コアには手を入れず、`plugins/` 配下のスクリプトを追加・変更するケースを想定しています。

## 1. 実行モデルの概要
```
Rust Pipeline --NDJSON--> plugins/lib/rule_host.py --Python import--> scripts/*.py
```
- Rust 側は 1 プロセスの Python ホストを起動し、`ruleset.scripts` に記載したファイルを import。
- 各ステージごとに `{"kind":"run_stage","stage":"ast","path":"...","payload":{...}}` を 1 行の JSON として送信。
- ホストは読み込んだ順に `module.check(req)` を呼び、返却された Violations を結合して Rust へ返す。
- Rust 終了時に `{"kind":"shutdown"}` を送信し、ホストは自身を停止。

## 2. `check(req)` の I/O 契約
### 2.1 リクエスト構造
```python
req = {
    "kind": "run_stage",
    "stage": "raw_text" | "pp_text" | "cst" | "ast",
    "path": "/abs/path/to/file.sv",
    "payload": <StagePayload>,
}
```
`payload` はステージごとに以下の型を取ります。

| ステージ | 内容 |
| --- | --- |
| `raw_text` | `{ "text": "..." }`（LF 正規化済みソース全文） |
| `pp_text` | `{ "text": "...", "defines": [{"name","value"}] }` |
| `cst` | `{ "mode": "inline", "cst_ir": {...} }` または `{ "mode": "none", "has_cst": bool }` |
| `ast` | `AstSummary`（`decls`, `refs`, `symbols`, `assigns`, `ports`, `pp_text` など） |

`AstSummary` や `cst_ir` の構造は [docs/internal_spec.md](internal_spec.md) を参照してください。

### 2.2 応答構造
`check` は Violation 辞書の配列を返します。返却値が `None` / 空リストの場合は「違反なし」と判定されます。

```python
return [{
    "rule_id": "format.line_length",
    "severity": "warning", # error / warning / info
    "message": "line exceeds 120 columns (134)",
    "location": {"line": 10, "col": 121, "end_line": 10, "end_col": 135}
}]
```
location の項目は 1-based、`end_*` は inclusive でも exclusive でも構いません（sv-mint 側でそのまま表示）。

### 2.3 例外処理
`check` から未処理例外を投げるとホストは `{ "type":"error" }` を Rust へ返し、そのステージ全体が失敗します。想定外ケースは例外ではなく Violation（`sys.rule.internal` など）として返すか、`try/except` で握りつぶしてください。

## 3. スケルトン例
```python
from typing import Any, Dict, List

RULE_ID = "example.rule"


def check(req: Dict[str, Any]) -> List[Dict[str, Any]]:
    if req.get("stage") != "ast":
        return []
    payload = req.get("payload") or {}
    symbols = payload.get("symbols") or []
    violations = []
    for sym in symbols:
        if sym.get("class") == "var" and sym.get("name", "").startswith("tmp_"):
            loc = sym.get("loc") or {"line": 1, "col": 1, "end_line": 1, "end_col": 1}
            violations.append({
                "rule_id": RULE_ID,
                "severity": "warning",
                "message": f"temporary signal {sym.get('name')} must be removed",
                "location": loc,
            })
    return violations
```
`plugins/debug_ping.py` も最小限のテンプレートとして参照できます。

## 4. デバッグ手法
- `sv-mint --config ... path` を実行し、`logging.show_plugin_events = true` にしておくと `PluginInvoke` / `PluginDone` のログでステージ時間を確認できます。
- ユニットテストは任意ですが、`pytest` 等で `check` を直接呼び出し JSON モックを与える方法が推奨されます。
- 一時的に print したい場合は stderr に出力し、`logging.stderr_snippet_bytes` を十分大きく設定すると CLI から確認できます。

## 5. 品質と運用のヒント
- ルール ID は `カテゴリ.名前` の形式に統一し、README/ユーザーガイドで検索しやすくします。
- 重い処理は極力 AST/CST をフィルタリングしてから行い、payload 全体を毎回コピーしないように注意してください。
- プロジェクト固有ルールを追加する場合は `plugins/` の下にサブフォルダを作り、`sv-mint.toml` の `ruleset.scripts` で絶対/相対パスを指定できます。仕様は `docs/plugins/<script_name>.md` へ追加し、利用者向けの情報を最新に保ってください。

## 6. 既知のサイズ/時間制約
- リクエスト JSON が 16 MB を超えるとステージが強制停止（required stage はエラー）。大型 payload を扱うルールでは、不要フィールドの削除やレポートのサマリ化を検討してください。
- `timeout_ms_per_file` は 1 ファイル全ステージの合計タイムアウトなので、単一ルールで時間を独占しないように設計します。

内部アーキテクチャ（Pipeline・サイズガード・イベントシステムなど）は [docs/internal_spec.md](internal_spec.md) を併せて参照してください。
