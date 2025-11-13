# seq_blocking_in_alwaysff.py

- **対応スクリプト**: `plugins/seq_blocking_in_alwaysff.py`
- **使用ステージ**: `cst`
- **主な入力フィールド**: `cst_ir`（`tokens`, `tok_kind_table`, `line_starts`, `pp_text`）
- **提供ルール**:
  | Rule ID | Severity | 動作概要 |
  | --- | --- | --- |
  | `seq.blocking_in_alwaysff` | warning | `always_ff` ブロックでのブロッキング代入 `=` を検知 |

## ルール詳細

### `seq.blocking_in_alwaysff`
- **検出条件**: `always_ff` 範囲を特定し、`op_eq` トークン（または `=` パターン）を見つけた位置で違反を生成します。
- **代表メッセージ**: `` blocking '=' inside always_ff ``
- **主な対処**: クロックドプロセス内の代入は `<=` を使うか、組み合わせロジックへ切り出します。
- **補足**: `sv-parser` のバージョン差異に対応するため、トークン欠落時は正規表現によるバックアップ走査を行っています。
