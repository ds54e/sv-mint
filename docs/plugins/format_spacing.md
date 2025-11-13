# format_spacing.py

- **対応スクリプト**: `plugins/format_spacing.py`
- **使用ステージ**: `raw_text` と `cst`
- **主な入力フィールド**: `text`（生テキスト）、`cst_ir`（`pp_text`, `line_starts`）
- **提供ルール**:
  | Rule ID | Severity | 動作概要 |
  | --- | --- | --- |
  | `format.comma_space` | warning | カンマの直後にスペースが無い箇所を検知 |
  | `format.call_spacing` | warning | 関数/タスク呼び出しで識別子と `(` の間のスペースを禁止 |
  | `format.macro_spacing` | warning | マクロ呼び出しで `(` 直前のスペースを禁止 |
  | `format.case_colon_spacing` | warning | `case` ラベルで `:` の前に空白がある場合を検知 |
  | `format.case_colon_after` | warning | `case` ラベルで `:` の後にスペースが無い場合を検知 |

## ルール詳細

### `format.comma_space`
- **検出条件**: 正規表現 `,(?!\s)` でスペース無しのカンマを探し、カラム位置を報告します。
- **代表メッセージ**: `` missing space after comma ``
- **主な対処**: 引数や配列要素を `, ` で区切り、読みやすいリスト表記に統一してください。

### `format.call_spacing`
- **検出条件**: `foo (` のように関数名と `(` の間にスペースがある場合を検知し、SystemVerilog の呼び出しスタイルに合わせます。予約語や宣言部 (`function foo (`) は除外されます。
- **代表メッセージ**: `` function or task call must not have space before '(' ``
- **主な対処**: 呼び出し箇所を `foo(` 形式へ修正します。

### `format.macro_spacing`
- **検出条件**: `
`macro (` のようにマクロ名と `(` の間が空いている場合を検知します。
- **代表メッセージ**: `` macro invocation must not have space before '(' ``
- **主な対処**: マクロ呼び出しを `` `FOO(`` の形に統一します。

### `format.case_colon_spacing` / `format.case_colon_after`
- **検出条件**: `cst` モードで `case` ブロック内の行を解析し、ラベル直前・直後の空白有無を確認します。
- **代表メッセージ**:
  - `` case item must not have whitespace before ':' ``
  - `` case item must have space after ':' ``
- **主な対処**: `case` の条件ラベルを `label: statement;` のように整形し、可変ホワイトスペースを統一します。
