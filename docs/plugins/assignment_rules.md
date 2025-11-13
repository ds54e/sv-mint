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
- **LowRISC 参照**: lowRISC SystemVerilog スタイルガイド（Sequential Logic Process セクション）は、同一フロップへの `<=` は 1 箇所に限定することを推奨しており、本ルールはその要件と一致します。
- **良い例**:

```systemverilog
always_ff @(posedge clk_i or negedge rst_ni) begin
  if (!rst_ni) begin
    state_q <= IDLE;
  end else begin
    state_q <= state_d;  // 代入箇所は 1 つ
  end
end
```

- **悪い例**:

```systemverilog
always_ff @(posedge clk_i) begin
  state_q <= state_d;
  if (flush_i) state_q <= IDLE;  // 同一クロックで 2 回目の <=
end
```

- **追加のポイント**: `generate` ブロック内部でも `(module, lhs)` 単位で追跡するため、`genvar` が異なるだけの繰り返しでも競合として検出されます。`unique case` などで `state_d` を決定し、`<=` は最後に 1 回だけ実行する構造へ寄せると誤検出を避けられます。
