# assignment_rules.py

- **対応スクリプト**: `plugins/assignment_rules.py`
- **使用ステージ**: `ast`
- **主な入力フィールド**: `assigns`（各代入の `module` / `lhs` / `op` / 位置情報）
- **提供ルール**:
  | Rule ID | Severity | 動作概要 |
  | --- | --- | --- |
  | `flow.multiple_nonblocking` | warning | 同一モジュールで同じ LHS へ複数の非ブロッキング代入がある場合に指摘 |

## ルール詳細

### `flow.multiple_nonblocking`
- **検出条件**: AST で収集した `assigns` のうち `op == nonblocking` を `(module, lhs)` 単位でグループ化し、2 件目以降の代入を違反として報告します。
- **代表メッセージ**: `` multiple nonblocking assignments to <lhs> ``
- **主な対処**: 1 クロック領域で 1 箇所だけ `<=` が残るようにロジックを整理するか、意図的に複数回代入している場合は片方を `=` で記述しない設計へ書き換えます。
- **補足**: `sv-mint.toml` でルールを無効化しない限り、階層をまたいだ競合も集計されるため、生成コードで意図せず重複が入っていないか確認してください。
