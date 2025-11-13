# global_define_rules.py

- **対応スクリプト**: `plugins/global_define_rules.py`
- **使用ステージ**: `raw_text`
- **主な入力フィールド**: `text`
- **提供ルール**:
  | Rule ID | Severity | 動作概要 |
  | --- | --- | --- |
  | `global.local_define_undef` | warning | ローカル用途のマクロは定義と同じファイルで `undef` することを要求 |
  | `global.prefer_parameters` | warning | 先頭が `_` 以外の `define` を禁止し、`parameter` 利用を推奨 |

## ルール詳細

### `global.local_define_undef`
- **検出条件**: `_FOO` のようなローカルマクロが `undef` されずにファイル末尾まで残っている場合に指摘します。
- **代表メッセージ**: `` local macro <name> must be undefined after use ``
- **主な対処**: 定義と同じ翻訳単位で `` `undef <name>`` を追加するか、より狭いスコープへ移します。
- **LowRISC 参照**: lowRISC のマクロ規約はローカルマクロに `_` 接頭辞を付け、翻訳単位の最後で確実に `undef` するよう求めています。
- **良い例**:

```systemverilog
`define _FOO(ARG) (ARG + 1)
assign data_o = `_FOO(data_i);
`undef _FOO
```

- **悪い例**:

```systemverilog
`define _FOO(ARG) (ARG + 1)
assign data_o = `_FOO(data_i);
// `undef されず、他ファイルへリーク
```

- **追加のポイント**: `include` 先で `undef` する際はシンボル名の衝突を防ぐため、`ifdef _FOO` ガードを挟むと安全です。

### `global.prefer_parameters`
- **検出条件**: `_` で始まらない `define` を検出し、設計全体に影響するマクロ乱用を抑止します。
- **代表メッセージ**: `` use parameters instead of global macro `FOO``
- **主な対処**: モジュールパラメータや `localparam` へ置き換え、`ruleset.override` で重大度を下げたい場合はポリシーに合わせて調整します。
- **LowRISC 参照**: lowRISC スタイルガイドは機能切り替えにグローバルマクロではなくパラメータを使うよう明示しており、必要最小限のトップレベルマクロしか許容しません。
- **良い例**:

```systemverilog
module foo #(parameter bit EnableParity = 1'b1) (...);
```

- **悪い例**:

```systemverilog
`define ENABLE_PARITY 1
module foo (...);
  if (`ENABLE_PARITY) begin
    ...
  end
endmodule
```

- **追加のポイント**: 既存の IP で多数の `define` が必要な場合は、`ruleset.allowlist` で `^OPENTITAN_` のような接頭辞のみ許可し、低リスクなマクロだけ通過させる運用が効果的です。
