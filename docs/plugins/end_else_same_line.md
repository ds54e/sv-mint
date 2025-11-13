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
