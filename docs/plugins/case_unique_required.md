# case_unique_required.py

- **対応スクリプト**: `plugins/case_unique_required.py`
- **使用ステージ**: `cst`（`mode = inline`）
- **主な入力フィールド**: `cst_ir.tokens`, `tok_kind_table`, `line_starts`
- **提供ルール**:
  | Rule ID | Severity | 動作概要 |
  | --- | --- | --- |
  | `lang.case_requires_unique` | warning | `case` 文に `unique`/`priority` 修飾子が無い場合に推奨を表示 |

## ルール詳細

### `lang.case_requires_unique`
- **検出条件**: `CaseStatement` のトークン列を解析し、`case` キーワード直前に `unique` もしくは `priority` が無い場合に違反を作成します。
- **代表メッセージ**: `` case statements should use unique or priority ``
- **主な対処**: 網羅性を求める場合は `unique case`、優先度を示したい場合は `priority case` へ書き換えます。仕様上不要であればルールをオフにしてください。
- **補足**: `case` が連続する構造 (`case inside`) では最初の `case` のみに適用されます。必要に応じて個別の `case` に修飾子を追加してください。
