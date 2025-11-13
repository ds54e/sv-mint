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
- **主な対処**: 定義と同じ翻訳単位で `
`undef <name>` を追加するか、より狭いスコープへ移します。

### `global.prefer_parameters`
- **検出条件**: `_` で始まらない `define` を検出し、設計全体に影響するマクロ乱用を抑止します。
- **代表メッセージ**: `` use parameters instead of global macro `FOO``
- **主な対処**: モジュールパラメータや `localparam` へ置き換え、`ruleset.override` で重大度を下げたい場合はポリシーに合わせて調整します。
