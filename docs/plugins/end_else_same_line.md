# end_else_same_line.py

- **対応スクリプト**: `plugins/end_else_same_line.py`
- **使用ステージ**: `pp_text`
- **主な入力フィールド**: プリプロ整形済み `text`
- **提供ルール**:
  | Rule ID | Severity | 動作概要 |
  | --- | --- | --- |
  | `format.end_else_inline` | warning | `end` の直後で改行される `else` を禁止し、同一行に揃える |

## ルール詳細

### `format.end_else_inline`
- **検出条件**: `end` の後で空白→改行→空白のパターンを検出し、その直後に `else` が開始している場合に `else` 側の位置で警告します。
- **代表メッセージ**: `` else must be on the same line as the preceding end ``
- **主な対処**: `end else` の行を 1 行にまとめるか、`end` に接続する `end else begin` などの書式に統一してください。
- **補足**: コメントで行が分断されている場合は検出対象外です。スタイルガイド上、`end else` ブロックの視認性を重視しています。
- **LowRISC 参照**: lowRISC スタイルガイドも `else` を前の `end` と同一行に置くことを推奨しており、`end` と `else` の間に空行を入れない明文化されたスタイルです。
- **良い例**:

```systemverilog
if (req_i) begin
  data_q <= data_d;
end else begin
  data_q <= '0;
end
```

- **悪い例**:

```systemverilog
if (req_i) begin
  data_q <= data_d;
end
else begin
  data_q <= '0;
end
```

- **追加のポイント**: `end` 行末コメント（`end // state latch` 等）がある場合は `end else` の間に 2 つ目のスペースを確保し、`end // state latch else ...` のようにコメントが干渉しないよう整形してください。`pp_text` をそのまま解析するため、`ifdef` で `else` を分けていると検出から外れるケースがあります。
