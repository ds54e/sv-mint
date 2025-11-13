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
- **LowRISC 参照**: lowRISC スタイルガイドは引数区切りに半角スペースを入れるよう求めています。
- **良い例**:

```systemverilog
assign sum = add(a_i, b_i, c_i);
```

- **悪い例**:

```systemverilog
assign sum = add(a_i,b_i,c_i);
```

- **追加のポイント**: マクロ引数でも同じ判定が動作します。`{a,b,c}` のような連結でスペースを揃えたい場合は `format.spacing` ルールの抑制対象に追加してください。

### `format.call_spacing`
- **検出条件**: `foo (` のように関数名と `(` の間にスペースがある場合を検知し、SystemVerilog の呼び出しスタイルに合わせます。予約語や宣言部 (`function foo (`) は除外されます。
- **代表メッセージ**: `` function or task call must not have space before '(' ``
- **主な対処**: 呼び出し箇所を `foo(` 形式へ修正します。
- **LowRISC 参照**: lowRISC スタイルガイドは関数/タスク呼び出しで識別子と `(` を密着させることを規定しています。
- **良い例**:

```systemverilog
foo(a_i, b_i);
```

- **悪い例**:

```systemverilog
foo (a_i, b_i);
```

- **追加のポイント**: `foo
(` のように改行を挟むと別ルールで指摘されるため、長い引数リストは `foo(
    .a(a_i),
    .b(b_i)
)` のように改行位置を `(` の後へ移してください。

### `format.macro_spacing`
- **検出条件**: `` `macro (`` のようにマクロ名と `(` の間が空いている場合を検知します。
- **代表メッセージ**: `` macro invocation must not have space before '(' ``
- **主な対処**: マクロ呼び出しを `` `FOO(`` の形に統一します。
- **LowRISC 参照**: lowRISC スタイルガイドのマクロ節でも、プリプロマクロ名と `(` を密着させることが推奨されています。
- **良い例**:

```systemverilog
`MY_ASSERT(condition)
```

- **悪い例**:

```systemverilog
`MY_ASSERT (condition)
```

- **追加のポイント**: 可変引数マクロでも同様に適用されます。`macro` 名をパラメタライズする場合は `` `define FOO(NAME) ...`` のように `define` 側で調整してください。

### `format.case_colon_spacing` / `format.case_colon_after`
- **検出条件**: `cst` モードで `case` ブロック内の行を解析し、ラベル直前・直後の空白有無を確認します。
- **代表メッセージ**:
  - `` case item must not have whitespace before ':' ``
  - `` case item must have space after ':' ``
- **主な対処**: `case` の条件ラベルを `label: statement;` のように整形し、可変ホワイトスペースを統一します。
- **LowRISC 参照**: lowRISC の case スタイル規約ではラベル名と `:` は密着させ、`:` の後には単一スペースを入れることが記されています。
- **良い例**:

```systemverilog
case (state_q)
  IDLE:   state_d = START;
  START:  state_d = DONE;
endcase
```

- **悪い例**:

```systemverilog
case (state_q)
  IDLE   :state_d = START;  // `:` の前後のスペースが逆
endcase
```

- **追加のポイント**: `localparam` や `enum` の `:` と混同しないよう、`cst` モードでは `CaseItem` に限定して解析しています。コメントを `:` の直後に置く必要がある場合は `IDLE: // comment` のようにスペース→コメントの順序を守ってください。
