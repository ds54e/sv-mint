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
- **LowRISC 参照**: lowRISC では 1 ファイル 1 パッケージを原則としており、テスト用の補助パッケージも別ファイルへ分けるよう求めています。
- **良い例**: `prim_pkg.sv` に `package prim_pkg; ... endpackage : prim_pkg` のみが定義されている。
- **悪い例**: 同じファイルに `package prim_pkg;` と `package prim_test_pkg;` を併記。
- **追加のポイント**: `interface` と `package` を同居させる場合は、`sv-mint.toml` で例外指定するかファイルを分けてください。

### `package.missing_end`
- **検出条件**: `package` が存在するのに `endpackage` がファイル内に無い場合に違反。
- **代表メッセージ**: `` package foo missing endpackage ``
- **主な対処**: `endpackage : foo` を追記します。
- **LowRISC 参照**: lowRISC スタイルガイドも `endpackage : name` を省略しないよう規定しています。
- **良い例**:

```systemverilog
package foo_pkg;
  typedef enum logic [1:0] { A, B } foo_e;
endpackage : foo_pkg
```

- **悪い例**:

```systemverilog
package foo_pkg;
  typedef enum logic [1:0] { A, B } foo_e;
// endpackage が欠落
```

- **追加のポイント**: `endpackage` を条件コンパイルに含めると欠落しやすいので、`ifdef` はパッケージ本文内部に限定します。

### `package.end_mismatch`
- **検出条件**: `endpackage : bar` のラベルが先頭宣言 `package foo` と一致しない場合に警告します。
- **代表メッセージ**: `` endpackage label bar does not match package foo ``
- **主な対処**: ラベルを正しい名前に修正。
- **LowRISC 参照**: lowRISC でも `endpackage : <name>` を必須とし、宣言名と同じラベルを使うよう明文化しています。
- **良い例**: `package prim_pkg; ... endpackage : prim_pkg`
- **悪い例**: `package prim_pkg; ... endpackage : foo_pkg`
- **追加のポイント**: 自動生成で `endpackage` ラベルをテンプレート化しておくとミスマッチを防げます。

### `package.define_in_package`
- **検出条件**: `package ... endpackage` の本文に現れる `` `define`` を調べ、`_` で始まらないマクロを警告します。
- **代表メッセージ**: `` prefer parameters over `define NAME inside package ``
- **主な対処**: パッケージ内では `parameter` / `localparam` を使用してください。
- **LowRISC 参照**: lowRISC はパッケージでマクロを定義することを避け、型や定数は `parameter` として公開するガイドラインです。
- **良い例**:

```systemverilog
package foo_pkg;
  parameter int FooDepth = 16;
endpackage : foo_pkg
```

- **悪い例**:

```systemverilog
package foo_pkg;
  `define FOO_DEPTH 16
endpackage : foo_pkg
```

- **追加のポイント**: 既存コードでマクロを使っている場合は `localparam` へ段階的に移行し、`import foo_pkg::*;` で参照する構造へ置き換えましょう。
