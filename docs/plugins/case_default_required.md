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
- **LowRISC 参照**: lowRISC SystemVerilog スタイルガイド（Case Statements 節）では、`unique case` であっても `default` を省略しない方針を取っており、異常値を握りつぶさないよう `default:` で `state_d = state_q;` などを明記することを勧めています。
- **良い例**:

```systemverilog
unique case (state_q)
  IDLE:   state_d = START;
  START:  state_d = DONE;
  default: state_d = IDLE;  // 例外経路を明示
endcase
```

- **悪い例**:

```systemverilog
case (opcode_i)
  4'h0: alu_d = ADD;
  4'h1: alu_d = SUB;
endcase  // default が無いため不定値を見逃す
```

- **追加のポイント**: `default` を `begin/end` で囲む場合、`default` トークンの直後に `:` が必要です。プリプロセッサディレクティブで `default` を挿入する場合も、`case` 本体と同じインデントに揃えておくと差分レビュー時に見落としを防げます。
