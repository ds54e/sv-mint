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
- **LowRISC 参照**: lowRISC SystemVerilog スタイルガイドは `case` をデフォルトで `unique case` にする方針を掲げており、優先度が必要な場合のみ `priority case` を許容しています。本ルールはその整合性を確認します。
- **良い例**:

```systemverilog
unique case (opcode_i)
  OP_ADD: res_d = a_i + b_i;
  OP_SUB: res_d = a_i - b_i;
  default: res_d = '0;
endcase
```

- **悪い例**:

```systemverilog
case (opcode_i)
  OP_ADD: res_d = a_i + b_i;
  OP_SUB: res_d = a_i - b_i;
  default: res_d = '0;
endcase  // unique/priority の指定がなく網羅性が不明
```

- **追加のポイント**: `priority case` は `default` が不要と誤解されがちですが、lowRISC フローでは安全側に倒して `default` を残すことが推奨です。`priority case` を用いる際は `priority if` への書き換えも検討すると、ロジックの意図が明確になります。
