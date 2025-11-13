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
- **LowRISC 参照**: lowRISC スタイルガイドは case アイテム内で複数行を記述する場合に `begin/end` を必須とし、単文であっても将来行が増えるなら先に `begin` を入れることを勧めています。
- **良い例**:

```systemverilog
unique case (state_q)
  IDLE: begin
    ready_o = 1'b1;
    state_d = START;
  end
  default: begin
    ready_o = 1'b0;
  end
endcase
```

- **悪い例**:

```systemverilog
case (state_q)
  START: ready_o = 1'b1;
          state_d = RUN;  // begin/end なしで複文
endcase
```

- **追加のポイント**: `case inside` でも同じルールが適用されます。`begin` を省略した箇所へ行コメントを差し込むと誤判定の原因になるため、`begin : label_name` のようにラベルを付けておくと解析が安定します。
