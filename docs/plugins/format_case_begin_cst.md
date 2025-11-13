# format_case_begin_cst.py

- **対応スクリプト**: `plugins/format_case_begin_cst.py`
- **使用ステージ**: `cst`（`mode = inline`）
- **主な入力フィールド**: `cst_ir.tokens`, `tok_kind_table`, `line_starts`
- **提供ルール**:
  | Rule ID | Severity | 動作概要 |
  | --- | --- | --- |
  | `format.case_begin_required` | warning | `case` の各アイテムで複文を `begin/end` で囲うことを強制 |

## ルール詳細

### `format.case_begin_required`
- **検出条件**: `CaseStatement` ノードの `:`（`colon` トークン）直後に現れる最初の非コメントトークンが `begin` でない場合に警告します。
- **代表メッセージ**: `` case item should wrap statements in begin/end ``
- **主な対処**: `case` の各ラベルに複数文が続く場合、`begin ... end` を追加してブロックを明示します。
- **補足**: `begin` で始まる単文はスキップされます。`unique case` との併用時も同じポリシーです。
