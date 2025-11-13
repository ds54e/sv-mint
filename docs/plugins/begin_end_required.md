# begin_end_required.py

- **対応スクリプト**: `plugins/begin_end_required.py`
- **使用ステージ**: `pp_text`
- **主な入力フィールド**: プリプロセス後テキスト `text`
- **提供ルール**:
  | Rule ID | Severity | 動作概要 |
  | --- | --- | --- |
  | `format.begin_required` | warning | 改行を挟む制御構文の本体を `begin ... end` で囲うことを強制 |

## ルール詳細

### `format.begin_required`
- **検出条件**: `if/for/foreach/while/repeat/forever` を走査し、本体が改行を含んでいるにもかかわらず `begin` で始まっていない場合に違反を生成します。
- **代表メッセージ**: `` <keyword> body must start with begin when split across lines ``
- **主な対処**: 条件の後に `begin` を挿入し、対応する `end` を追加します。単文なら同一行で維持するか、`begin/end` を明示して可読性を保ってください。
- **補足**: `else if` 連鎖は `if` キーワード検査時に `else` を考慮しているため、`else` 側の `begin` も忘れずにそろえる必要があります。
- **LowRISC 参照**: lowRISC SystemVerilog スタイルガイドの Control Flow 章では、複数行に分かれる制御本体は必ず `begin`/`end` で明確化することを義務付けています。
- **良い例**:

```systemverilog
if (req_i) begin
  data_q <= data_d;
  cnt_q  <= cnt_q + 1;
end else begin
  cnt_q <= '0;
end
```

- **悪い例**:

```systemverilog
if (req_i)
  data_q <= data_d;
  cnt_q  <= cnt_q + 1;  // begin/end が無くブロックが曖昧
```

- **追加のポイント**: `begin/end` を入れると `/* verilator lint_off */` のような行頭ディレクティブと干渉する場合があります。プリプロセス後テキストを対象にしているため、マクロでラップしている場合は最終展開形で `begin` が現れるようにしてください。
