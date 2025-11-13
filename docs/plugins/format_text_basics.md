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
- **LowRISC 参照**: lowRISC のテキスト規約は SV ファイルを ASCII のみで構成するよう求めています。多言語コメントが必要な場合は `docs/` に切り出す運用です。
- **良い例**:

```systemverilog
// state machine controls DMA start
```

- **悪い例**:

```systemverilog
// 状態機械開始  ← 非 ASCII の全角文字が混入
```

- **追加のポイント**: 自動生成コメントに日本語が入るケースでは `format_text_basics` を一時的に無効化できますが、翻訳済み資料を `docs/` 側に置く方が低リスクです。

### `format.no_tabs`
- **検出条件**: タブ文字 `	` が現れるたびに違反を生成します。
- **代表メッセージ**: `` tab character detected ``
- **主な対処**: タブをスペースへ置換し、`format_indent_rules` で定義された幅に従います。
- **LowRISC 参照**: lowRISC スタイルガイドは「タブ禁止」を明確に掲げ、ツール間の表示差異を排除しています。
- **良い例**:

```systemverilog
logic ready;
```

- **悪い例**:

```systemverilog
	logic ready;
```

- 行頭にタブが含まれているためツール間で表示位置がずれます。

- **追加のポイント**: `.editorconfig` の `indent_style = space` を併用すると自動保存時にタブが発生しません。`tab` を許容したいテストデータは `allowlist` にピン留めしてください。

### `format.no_trailing_whitespace`
- **検出条件**: 各行の末尾から逆走査し、空白/タブで終わっている場合に列位置を報告します。
- **代表メッセージ**: `` trailing whitespace at line end ``
- **主な対処**: 保存時にトリムするか、エディタフックで自動除去します。
- **LowRISC 参照**: lowRISC では行末空白を禁止し、`git diff --check` を常用することが推奨されています。
- **良い例**:

```systemverilog
assign ready_o = valid_i;
```

- **悪い例**:

```systemverilog
assign ready_o = valid_i;␠
```

- **追加のポイント**: `sv-mint` は LF 正規化後のデータを解析するため、CRLF 混在でも問題無く列情報を返します。`pre-commit` で `trailing-whitespace` フックを入れると CI 前に捕捉できます。

### `format.final_newline`
- **検出条件**: ファイル末尾が `\n` で終わらない場合に警告します。
- **代表メッセージ**: `` file must end with newline ``
- **主な対処**: 最終行の後に改行を追加します。
- **LowRISC 参照**: lowRISC も POSIX 準拠のため EOF に LF を置くことを求めています。
- **良い例**: `endmodule` の後に空行を 1 つ置いてファイルを終える。
- **悪い例**: `endmodule` でファイルが終わっており、最終行に LF が無い状態。
- **追加のポイント**: `git` は最終行が改行で閉じられていないと差分末尾に `\ No newline at end of file` を出力します。CI でノイズが出る前に `format.final_newline` で検知できます。
