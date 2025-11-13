# naming_rules.py

- **対応スクリプト**: `plugins/naming_rules.py`
- **使用ステージ**: `ast`
- **主な入力フィールド**: `decls`, `symbols`, `ports`
- **提供ルール**:
  | Rule ID | Severity | 動作概要 |
  | --- | --- | --- |
  | `naming.module_case` | warning | モジュール名を lower_snake_case に統一 |
  | `naming.signal_case` | warning | 信号/変数名の snake_case を強制 |
  | `naming.port_case` | warning | ポート名の snake_case を強制 |
  | `naming.no_numeric_suffix` | warning | `_42` のような末尾数値サフィックスを禁止 |
  | `naming.suffix_order` | warning | `_n_i` などサフィックスの順序を統一（`_ni/_no/_nio`） |
  | `naming.clk_prefix` | warning | クロック名は `clk` から始める |
  | `naming.rst_active_low` | warning | リセット名は `_n` などで負論理を示す |
  | `naming.clk_order` | warning | ポート順序で `clk` を先頭に置く |
  | `naming.rst_before_clk` | warning | `rst` を `clk` より前に書かない |
  | `naming.differential_pair` | warning | `_p` と `_n` の差動ポートをペアで定義 |
  | `naming.pipeline_sequence` | warning | `_q2` 以降のパイプライン段は前段を必須とする |
  | `naming.parameter_upper` | warning | `parameter` 名を UpperCamelCase に統一 |

## ルール詳細

### `naming.module_case`
- **検出条件**: `decls` のモジュール宣言名が lower_snake_case でない場合に報告。
- **代表メッセージ**: `` module FooBar must be lower_snake_case ``
- **主な対処**: `foo_bar` のように書式を修正。

### `naming.signal_case` / `naming.port_case`
- **検出条件**: 信号やポートを lower_snake_case に統一し、CamelCase や大文字を禁止します。
- **代表メッセージ**: `` signal fooBar must use lower_snake_case ``
- **主な対処**: 命名ルールへ従ってリネーム。

### `naming.no_numeric_suffix`
- **検出条件**: `_` に続く整数で終わる名前を検知します。
- **代表メッセージ**: `` foo_1 must not end with _<number> ``
- **主な対処**: `_stage1` ではなく `_s1` 等の別表現に変更。

### `naming.suffix_order`
- **検出条件**: `_n_i` のようにアクティブローと方向サフィックスが分断されている名前を検出。
- **代表メッセージ**: `` foo_n_i must combine suffixes without extra underscores ``
- **主な対処**: `_ni/_no/_nio` 形式で一体化。

### `naming.clk_prefix`
- **検出条件**: `clk` を含むが `clk` で始まらない名前を警告。
- **代表メッセージ**: `` name must start with 'clk' ``
- **主な対処**: `clk_core` などへ変更。

### `naming.rst_active_low`
- **検出条件**: `rst` で始まるにも関わらず `_n`（または `_ni/_no/_nio`）で終わらない場合に報告。
- **代表メッセージ**: `` name must end with '_n' for active-low resets ``
- **主な対処**: リセット名を `rst_n` 系に統一。

### `naming.clk_order` / `naming.rst_before_clk`
- **検出条件**: ポート一覧を順番に走査し、`clk` が最初に並び、`rst` がその後に続いているか確認します。
- **代表メッセージ**:
  - `` clk ports should appear before resets and other ports ``
  - `` rst ports should follow clk ports ``
- **主な対処**: モジュールポートの宣言順序を調整。

### `naming.differential_pair`
- **検出条件**: `_p` のポートに `_n` が存在しない場合に発報。
- **代表メッセージ**: `` differential pair missing counterpart for foo_p ``
- **主な対処**: 対応する `_n` ポートを追加するか、差動で無いなら `_p` 命名を避けます。

### `naming.pipeline_sequence`
- **検出条件**: `_q2` 以降のレジスタ名を見つけたとき、直前段 (`_q` / `_q1` / `_q<n-1>`) が未定義なら警告。
- **代表メッセージ**: `` pipeline stage foo_q3 missing previous stage foo_q2 ``
- **主な対処**: 連番が途切れないように宣言し、未使用段を削除します。

### `naming.parameter_upper`
- **検出条件**: `decls` の `param` で先頭が大文字でない場合に違反。
- **代表メッセージ**: `` parameter width should use UpperCamelCase ``
- **主な対処**: `DataWidth` など UpperCamelCase 名へ変更。
