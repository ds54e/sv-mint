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
- **LowRISC 参照**: lowRISC スタイルガイドは SystemVerilog のインデント幅を 2 スペースに固定するよう定めています。
- **良い例**:

```systemverilog
module foo;
  logic req;
  always_comb begin
    if (req) data = '0;
  end
end
```

- **悪い例**:

```systemverilog
module foo;
   logic req;   // 3 スペースでずれている
    always_comb begin
      if (req) data = '0;
    end
end
```

- **追加のポイント**: タブ混在を排除するため、エディタ側で `expandtab` を有効にし、`sv-mint` の診断をトリガーに `.editorconfig` を整備すると揃え漏れを防げます。

### `format.preproc_left_align`
- **検出条件**: 行頭がスペースやタブで始まる `define/ifdef/ifndef/endif` などのプリプロ命令を検出し、列 1 に揃えるよう指示します。
- **代表メッセージ**: `` preprocessor directives must be left aligned ``
- **主な対処**: タブやスペースを削除し、``define/ifdef`` などを行頭に配置します。
- **LowRISC 参照**: lowRISC のプリプロセッサ規約でも、インデントを付けずに列頭へ配置することが明記されています。
- **良い例**:

```systemverilog
`ifdef INCLUDE_DBG
  assign dbg_o = dbg_i;
`endif
```

- **悪い例**:

```systemverilog
  `ifdef INCLUDE_DBG
    assign dbg_o = dbg_i;
  `endif
```

- **追加のポイント**: ネストが深くても左端に置く方針は不変です。どうしても字下げしたい場合は `// DEBUG` のようなコメントを行末へ足して文脈を補足してください。

### `format.line_continuation_right`
- **検出条件**: 行末に `\` を含むにもかかわらず末尾が空白で終わっている（`\` の右に文字がある）場合に警告します。
- **代表メッセージ**: `` line continuation \ must be last character ``
- **主な対処**: 継続が必要な行では `\` を最後の文字にし、コメントや空白を右側に残さないようにします。
- **LowRISC 参照**: lowRISC ガイドでもマクロ継続記号を行末に置くことが推奨され、`\` の右に空白を残さない方針です。
- **良い例**:

```systemverilog
`define BUS_FIELDS \
  logic req;      \
  logic gnt;
```

- **悪い例**:

```systemverilog
`define BUS_FIELDS \
  logic req; \   // バックスラッシュの後に空白が残る
  logic gnt;
```

- **追加のポイント**: 行末空白は `git diff --check` でも検出されますが、`sv-mint` の診断は `pp_text` ベースで発火するため、タブ混在があると実際の列位置とずれる点に注意してください。
