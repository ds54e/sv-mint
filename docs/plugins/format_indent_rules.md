# format_indent_rules.py

- **対応スクリプト**: `plugins/format_indent_rules.py`
- **使用ステージ**: `raw_text`
- **主な入力フィールド**: `text`（生テキスト）
- **提供ルール**:
  | Rule ID | Severity | 動作概要 |
  | --- | --- | --- |
  | `format.indent_multiple_of_two` | warning | ブロックのインデント幅を 2 の倍数に統一 |
  | `format.preproc_left_align` | warning | `define/ifdef/endif` などプリプロ命令を行頭に揃える |
  | `format.line_continuation_right` | warning | バックスラッシュによる行継続は行末で終わる必要がある |

## ルール詳細

### `format.indent_multiple_of_two`
- **検出条件**: 空白のみを取り除いた文字列の長さからインデント幅を算出し、奇数のスペース数だった行を報告します。
- **代表メッセージ**: `` indentation should be multiples of 2 spaces ``
- **主な対処**: `TAB` ではなくスペース 2 の倍数へ統一し、コードブロック内の揃え方を再確認してください。

### `format.preproc_left_align`
- **検出条件**: `	` を含まない状態で `	` もしくはスペースでインデントされたプリプロ命令行を検出し、列 1 に揃えるよう指示します。
- **代表メッセージ**: `` preprocessor directives must be left aligned ``
- **主な対処**: `	` やスペースを削除し、` `define/ifdef` などを行頭に配置します。

### `format.line_continuation_right`
- **検出条件**: 行末に `\` を含むにもかかわらず末尾が空白で終わっている（`\` の右に文字がある）場合に警告します。
- **代表メッセージ**: `` line continuation \ must be last character ``
- **主な対処**: 継続が必要な行では `\` を最後の文字にし、コメントや空白を右側に残さないようにします。
