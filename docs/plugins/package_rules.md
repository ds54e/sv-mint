# package_rules.py

- **対応スクリプト**: `plugins/package_rules.py`
- **使用ステージ**: `raw_text`
- **主な入力フィールド**: `text`
- **提供ルール**:
  | Rule ID | Severity | 動作概要 |
  | --- | --- | --- |
  | `package.multiple` | warning | 1 ファイル内に複数の `package` 宣言がある場合に指摘 |
  | `package.missing_end` | warning | `endpackage` が欠落している場合に警告 |
  | `package.end_mismatch` | warning | `endpackage : foo` のラベルが `package` 名と一致しない場合を検知 |
  | `package.define_in_package` | warning | `package` ブロック内の `define` 利用を禁止 |

## ルール詳細

### `package.multiple`
- **検出条件**: 正規表現で `package` キーワードをカウントし、2 回以上登場した場合に 1 件目の位置を報告します。
- **代表メッセージ**: `` multiple package declarations in single file (pkg_name) ``
- **主な対処**: パッケージごとにファイルを分割するか、別名として整理します。

### `package.missing_end`
- **検出条件**: `package` が存在するのに `endpackage` がファイル内に無い場合に違反。
- **代表メッセージ**: `` package foo missing endpackage ``
- **主な対処**: `endpackage : foo` を追記します。

### `package.end_mismatch`
- **検出条件**: `endpackage : bar` のラベルが先頭宣言 `package foo` と一致しない場合に警告します。
- **代表メッセージ**: `` endpackage label bar does not match package foo ``
- **主な対処**: ラベルを正しい名前に修正。

### `package.define_in_package`
- **検出条件**: `package ... endpackage` の本文に現れる `` `define`` を調べ、`_` で始まらないマクロを警告します。
- **代表メッセージ**: `` prefer parameters over `define NAME inside package ``
- **主な対処**: パッケージ内では `parameter` / `localparam` を使用してください。
