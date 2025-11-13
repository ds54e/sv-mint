# case_default_required.py

- **対応スクリプト**: `plugins/case_default_required.py`
- **使用ステージ**: `cst`（`mode = inline`）
- **主な入力フィールド**: `cst_ir.tokens`, `line_starts`, `pp_text`
- **提供ルール**:
  | Rule ID | Severity | 動作概要 |
  | --- | --- | --- |
  | `case.missing_default` | warning | `case` 文に `default` ラベルが存在しない場合に警告 |

## ルール詳細

### `case.missing_default`
- **検出条件**: CST の `CaseStatement` ノードを走査し、トークン列に `default` が現れない場合に先頭トークン位置で違反を生成します。
- **代表メッセージ**: `` case statement must include a default item ``
- **主な対処**: 網羅性を証明できる `unique case` でない限り `default` 節を追加し、意図的にフォールスルーさせたい場合も `default: <noop>` 等で明示します。
- **補足**: マクロ展開後の `pp_text` を走査しているため、`default` をマクロで定義している場合は前処理結果に現れる形にしてください。
