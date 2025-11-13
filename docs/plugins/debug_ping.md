# debug_ping.py

- **対応スクリプト**: `plugins/debug_ping.py`
- **使用ステージ**: 任意（呼び出し時の `stage` をそのまま表示）
- **主な入力フィールド**: `payload.ast` / `payload.symbols` など AST 由来の統計
- **提供ルール**:
  | Rule ID | Severity | 動作概要 |
  | --- | --- | --- |
  | `debug.ping` | warning | 受け取ったステージ名とシンボル数をエコーバックし、パイプライン疎通を確認 |

## ルール詳細

### `debug.ping`
- **検出条件**: 常に 1 件の違反を生成し、`payload` 内で見つかった `symbols`（もしくは `ast.symbols`）の件数をメッセージに含めます。
- **代表メッセージ**: `` debug ping: stage=ast, symbols=42 ``
- **主な対処**: デバッグ専用なので本番ルールセットでは `sv-mint.toml` の `[ruleset.scripts]` から削除します。
- **補足**: ルール拡張時にデータが想定どおり届いているかを素早く確認する用途を想定しています。Severity は `to_viol` 呼び出し時に上書き可能です。
