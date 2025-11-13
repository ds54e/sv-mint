# format_text_basics.py

- **対応スクリプト**: `plugins/format_text_basics.py`
- **使用ステージ**: `raw_text`
- **主な入力フィールド**: `text`
- **提供ルール**:
  | Rule ID | Severity | 動作概要 |
  | --- | --- | --- |
  | `format.ascii_only` | warning | 非 ASCII 文字を禁止 |
  | `format.no_tabs` | warning | タブ文字を禁止 |
  | `format.no_trailing_whitespace` | warning | 行末の空白を検出 |
  | `format.final_newline` | warning | EOF に LF が無い場合に警告 |

## ルール詳細

### `format.ascii_only`
- **検出条件**: `ord(ch) > 127` の文字を見つけ次第、位置を報告します。
- **代表メッセージ**: `` non-ASCII character detected ``
- **主な対処**: コメントを含め ASCII 以外の文字を削除するか、UTF-8 許容ポリシーに合わせてルールをオフにします。

### `format.no_tabs`
- **検出条件**: タブ文字 `\t` が現れるたびに違反を生成します。
- **代表メッセージ**: `` tab character detected ``
- **主な対処**: タブをスペースへ置換し、`format_indent_rules` で定義された幅に従います。

### `format.no_trailing_whitespace`
- **検出条件**: 各行の末尾から逆走査し、空白/タブで終わっている場合に列位置を報告します。
- **代表メッセージ**: `` trailing whitespace at line end ``
- **主な対処**: 保存時にトリムするか、エディタフックで自動除去します。

### `format.final_newline`
- **検出条件**: ファイル末尾が `\n` で終わらない場合に警告します。
- **代表メッセージ**: `` file must end with newline ``
- **主な対処**: 最終行の後に改行を追加します。
