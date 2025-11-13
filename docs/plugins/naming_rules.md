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
- **LowRISC 参照**: lowRISC スタイルガイドはモジュール/インスタンスともに lower_snake_case を要求しています。
- **良い例**:

```systemverilog
module dma_ctrl (...);
```

- **悪い例**:

```systemverilog
module DmaCtrl (...);
```

- **追加のポイント**: ファイル名もモジュール名と一致させると `sv-mint` の他ルールと整合します。

### `naming.signal_case` / `naming.port_case`
- **検出条件**: 信号やポートを lower_snake_case に統一し、CamelCase や大文字を禁止します。
- **代表メッセージ**: `` signal fooBar must use lower_snake_case ``
- **主な対処**: 命名ルールへ従ってリネーム。
- **LowRISC 参照**: lowRISC では信号・ポートを lower_snake_case、末尾に方向サフィックス（`_i/_o/_io`）を付ける方針です。
- **良い例**:

```systemverilog
input  logic req_i;
output logic ack_o;
logic  data_valid;
```

- **悪い例**:

```systemverilog
input  logic Req;
output logic ACK;
logic  dataValid;
```

- **追加のポイント**: `sv-mint` は AST の宣言を基にリネーム候補を表示するため、`typedef struct packed` 内のフィールドでも同ルールが適用されます。

### `naming.no_numeric_suffix`
- **検出条件**: `_` に続く整数で終わる名前を検知します。
- **代表メッセージ**: `` foo_1 must not end with _<number> ``
- **主な対処**: `_stage1` ではなく `_s1` 等の別表現に変更。
- **LowRISC 参照**: lowRISC では末尾数字のみで区別する命名を避け、意味を持つサフィックス（`_a/_b` など）を付けることを推奨しています。
- **良い例**: `req_a`, `req_b` や `req_q1` のように役割や段数を明示。
- **悪い例**: `req_1`, `req_2` のような機械的な末尾数字。
- **追加のポイント**: パイプライン段を表す場合は `_q`, `_q1`, `_q2` のように `naming.pipeline_sequence` で許容される書式を利用してください。

### `naming.suffix_order`
- **検出条件**: `_n_i` のようにアクティブローと方向サフィックスが分断されている名前を検出。
- **代表メッセージ**: `` foo_n_i must combine suffixes without extra underscores ``
- **主な対処**: `_ni/_no/_nio` 形式で一体化。
- **LowRISC 参照**: lowRISC の命名規約も `_ni/_no/_nio` を推奨し、`rst_n_i` のような分断を禁止しています。
- **良い例**: `rst_ni`, `alert_no`。
- **悪い例**: `rst_n_i`, `alert_o_n`。
- **追加のポイント**: 自動変換時は `_n_i` を `_ni` へ正規化し、必要なら `svdep rename` 等で一括変更します。

### `naming.clk_prefix`
- **検出条件**: `clk` を含むが `clk` で始まらない名前を警告。
- **代表メッセージ**: `` name must start with 'clk' ``
- **主な対処**: `clk_core` などへ変更。
- **LowRISC 参照**: lowRISC ではクロック名を `clk_` で始め、方向サフィックスを末尾に付ける（例: `clk_i`）方針です。
- **良い例**: `input logic clk_i;`
- **悪い例**: `input logic core_clk_i;` や `logic hclk;` のように `clk` で始まらない名前。
- **追加のポイント**: `clk_core_i` のように `clk_` の後へドメイン名を入れると複数クロックでも区別しやすくなります。

### `naming.rst_active_low`
- **検出条件**: `rst` で始まるにも関わらず `_n`（または `_ni/_no/_nio`）で終わらない場合に報告。
- **代表メッセージ**: `` name must end with '_n' for active-low resets ``
- **主な対処**: リセット名を `rst_n` 系に統一。
- **LowRISC 参照**: lowRISC ではアクティブローリセットを `rst_ni`（入力）や `rst_no`（出力）で命名することが定められています。
- **良い例**: `input logic rst_ni;`
- **悪い例**: `input logic reset;` や `rst_i`。
- **追加のポイント**: アクティブハイの特殊ケースでは `ruleset.allowlist` で `rst_hi` などを例外登録してください。

### `naming.clk_order` / `naming.rst_before_clk`
- **検出条件**: ポート一覧を順番に走査し、`clk` が最初に並び、`rst` がその後に続いているか確認します。
- **代表メッセージ**:
  - `` clk ports should appear before resets and other ports ``
  - `` rst ports should follow clk ports ``
- **主な対処**: モジュールポートの宣言順序を調整。
- **LowRISC 参照**: lowRISC スタイルは `clk`→`rst`→その他の順にポートを並べることを要求します。
- **良い例**:

```systemverilog
module foo (
  input  logic clk_i,
  input  logic rst_ni,
  input  logic req_i,
  output logic ack_o
);
```

- **悪い例**: `req_i` や `data_i` を `clk_i` より前に並べたり、`rst_ni` を出力群の後ろへ置く並び。
- **追加のポイント**: `interface` ポートでも同じ順序が適用されます。自動生成時はテンプレートで順番を固定してください。

### `naming.differential_pair`
- **検出条件**: `_p` のポートに `_n` が存在しない場合に発報。
- **代表メッセージ**: `` differential pair missing counterpart for foo_p ``
- **主な対処**: 対応する `_n` ポートを追加するか、差動で無いなら `_p` 命名を避けます。
- **LowRISC 参照**: lowRISC でも差動信号は `_p` / `_n` のペアで宣言し、両方に方向サフィックスを付けるルールです（例: `i2c_scl_p_o`）。
- **良い例**: `output logic tx_p_o; output logic tx_n_o;`
- **悪い例**: `output logic tx_p_o;` のみで `_n` が欠けている。
- **追加のポイント**: `_n` が別モジュールで生成される場合は命名を合わせるか、例外として `allowlist` に信号名を登録してください。

### `naming.pipeline_sequence`
- **検出条件**: `_q2` 以降のレジスタ名を見つけたとき、直前段 (`_q` / `_q1` / `_q<n-1>`) が未定義なら警告。
- **代表メッセージ**: `` pipeline stage foo_q3 missing previous stage foo_q2 ``
- **主な対処**: 連番が途切れないように宣言し、未使用段を削除します。
- **LowRISC 参照**: lowRISC のレジスタ命名規約も `_q`, `_q1`, `_q2` で遅延段を明示し、段飛ばしを禁止しています。
- **良い例**: `data_q`, `data_q1`, `data_q2` と順番に宣言。
- **悪い例**: `data_q` と `data_q2` だけが存在し、`data_q1` が欠落。
- **追加のポイント**: パイプラインの一部を削除した際は `_q` 番号を再整理するか `*_d` の組み合わせで段数を表してください。

### `naming.parameter_upper`
- **検出条件**: `decls` の `param` で先頭が大文字でない場合に違反。
- **代表メッセージ**: `` parameter width should use UpperCamelCase ``
- **主な対処**: `DataWidth` など UpperCamelCase 名へ変更。
- **LowRISC 参照**: lowRISC のパラメータ命名規約は `NumAlerts`, `DataWidth` のような UpperCamelCase を推奨し、`WIDTH` といった全大文字は `localparam` のみに限定しています。
- **良い例**: `parameter int DataWidth = 32;`
- **悪い例**: `parameter int data_width = 32;` や `parameter int WIDTH = 32;`
- **追加のポイント**: 既存コードを一括変換する際は `sv-mint --autofix rename` を併用し、インスタンス側の上書き値も同時に変更してください。
