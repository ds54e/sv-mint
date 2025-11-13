# comb_nb_in_alwayscomb.py

- **対応スクリプト**: `plugins/comb_nb_in_alwayscomb.py`
- **使用ステージ**: `cst`
- **主な入力フィールド**: `cst_ir`（`tokens`, `tok_kind_table`, `line_starts`, `pp_text`）
- **提供ルール**:
  | Rule ID | Severity | 動作概要 |
  | --- | --- | --- |
  | `comb.nb_in_alwayscomb` | warning | `always_comb` ブロック内の非ブロッキング代入 (`<=`) を禁止 |

## ルール詳細

### `comb.nb_in_alwayscomb`
- **検出条件**: `AlwaysConstruct` のうち `always_comb` を特定し、そのトークン領域内で `<=`（`op_le`）が出現した位置を報告します。トークン情報が無い場合はテキスト走査にフォールバックします。
- **代表メッセージ**: `` nonblocking '<=' inside always_comb ``
- **主な対処**: 組み合わせ回路ではブロッキング代入 `=` へ変更し、状態保持が必要なら `always_ff` 等へ分離してください。
- **補足**: `sv-parser` のトークン種別が更新された場合は `tok_kind_table` に `op_le` が含まれていることを確認してください。
