# ルール詳細ガイド（ユーザー向け）

sv-mint が報告する SystemVerilog ルールを、実務で違反を確認したユーザーがすぐ対処できるよう再構成したリファレンスです。CLI から得た `rule_id` を起点に、ステージ・既定 Severity・修正の勘所をまとめています。プラグイン実装の深掘りは `docs/rule_reference.md` / `plugins/*.py` を参照してください。

## 0. 目的と想定読者
- 設計/検証エンジニアがレビュー中に `sv-mint` の診断を見た際、背景と対応策を 1 つのドキュメントで把握する。
- ルール運用担当が `sv-mint.toml` のオン/オフや重大度変更を検討する際の判断材料を提供する。
- CLI で出力される ID と本ドキュメントを相互参照しやすいよう、カテゴリとステージを整理する。

## 1. クイックナビ

| カテゴリ | 目的 | 代表ルール |
| --- | --- | --- |
| [3.1 フォーマット＆テキスト整形](#31-フォーマットテキスト整形) | 行単位のコーディングスタイルとプリプロ制約 | `format.line_length`, `format.no_tabs`, `format.indent_multiple_of_two` |
| [3.2 制御構造と case ブロック](#32-制御構造と-case-ブロック) | `if/case` など構造的ルール | `format.begin_required`, `case.missing_default`, `lang.case_requires_unique` |
| [3.3 プロセス/代入ルール](#33-プロセス代入ルール) | `always_*` ブロック内の代入種別 | `comb.nb_in_alwayscomb`, `seq.blocking_in_alwaysff`, `flow.multiple_nonblocking` |
| [3.4 インタフェース/ヘッダー/パッケージ](#34-インタフェースヘッダーパッケージ) | モジュール接続・マクロ・ファイル先頭 | `module.no_port_wildcard`, `global.prefer_parameters`, `header.missing_spdx` |
| [3.5 命名規約](#35-命名規約) | 信号・ポート・パラメータ命名と並び順 | `naming.module_case`, `naming.clk_prefix`, `naming.parameter_upper` |
| [3.6 宣言・型・リテラル](#36-宣言型リテラル) | 未使用宣言や typedef、リテラル幅 | `decl.unused.var`, `typedef.enum_suffix`, `width.unsized_base_literal` |
| [3.7 補助ルール](#37-補助ルール) | テンプレート検知やデバッグ用途 | `template.raw_text_marker`, `debug.ping` |

## 2. 使い方のヒント

### 2.1 ステージの見方

| ステージ | 観測対象 | 主な利用例 |
| --- | --- | --- |
| raw_text | 入力ファイルをそのまま参照 | 行長・ASCII 制約・プリプロ位置・`package` や `typedef` の検査 |
| pp_text | `sv-parser` でプリプロ済みテキスト | `if/else` の折返しや `end/else` のレイアウト検査 |
| cst (inline) | `sv-parser` の構文木（トークン位置込み） | `case` 内の `begin` 有無、`always_*` 範囲の演算子検出 |
| ast | 宣言・参照・シンボル情報 | 命名規約、未使用シンボル、複数代入の検出 |

### 2.2 Severity と運用
- 本表に記載の Severity は既定値（多くは `warning`）です。`sv-mint.toml` の `[ruleset.override]` に `rule_id = "error"` のように記述して上書きできます。
- 異なるステージで同じ `rule_id` が現れる場合はありません。重複定義を避けたい場合は `[ruleset] scripts` からプラグインを取り除いてください。
- プリプロや生成コードでの一時的な例外はローカル設定ファイルを分けて吸収し、共有リポジトリでは逸脱を残さない方針を推奨します。

## 3. カテゴリ別リファレンス

### 3.1 フォーマット＆テキスト整形

| ルールID | ステージ | Severity | チェック内容と対処 |
| --- | --- | --- | --- |
| format.line_length | raw_text | warning | 1 行 100 文字超を検出。長い式は変数抽出や改行で折り返し、コメントも 100 文字以内に収めます。 |
| format.ascii_only | raw_text | warning | ASCII 以外の文字を検知。全角や制御文字を避け、必要ならエスケープや UTF-8 コメントに置き換えます。 |
| format.no_tabs | raw_text | warning | タブ文字を禁止。スペース（既定は 2 or 4）へ変換し、エディタでタブ入力を抑止します。 |
| format.no_trailing_whitespace | raw_text | warning | 行末に空白が残る場合に警告。自動トリム設定を有効化してください。 |
| format.final_newline | raw_text | warning | 最終行の改行欠落を検出。POSIX 互換のためファイル末尾に LF を追記します。 |
| format.indent_multiple_of_two | raw_text | warning | インデントが 2 の倍数でない行を検知。フォーマッタやエディタ設定で固定幅に揃えます。 |
| format.preproc_left_align | raw_text | warning | `` `define`` や `` `ifdef`` が行頭以外から始まる場合に指摘。先頭の空白を削除し左寄せします。 |
| format.line_continuation_right | raw_text | warning | 行継続の `\` 右側に文字がある場合に警告。`\` を最後の文字にしてください。 |
| format.comma_space | raw_text | warning | カンマ直後に空白が無い箇所を検出。`, ` の形へ統一します。 |
| format.call_spacing | raw_text | warning | 関数/タスク名と `(` の間にスペースがある場合に警告。呼び出し名と括弧を隣接させます。 |
| format.macro_spacing | raw_text | warning | マクロ名と `(` の間の空白を検出。`` `FOO(bar)` に修正します。 |
| format.case_colon_spacing | cst (inline) | warning | `case` ラベルと `:` の前に空白がある場合に指摘。`label:` に整えます。 |
| format.case_colon_after | cst (inline) | warning | `:` の直後に空白が無い場合に警告。`label: stmt` のようにスペースを追加します。 |

### 3.2 制御構造と case ブロック

| ルールID | ステージ | Severity | チェック内容と対処 |
| --- | --- | --- | --- |
| format.begin_required | pp_text | warning | 複数行の `if/for/while/repeat/forever` 本体で `begin/end` が省略されている場合に警告。折り返す際は必ず `begin ... end` を追加します。 |
| format.end_else_inline | pp_text | warning | `end` の直後行に `else` がある場合を検出。`end else` を同一行に揃え、読み手の見落としを防ぎます。 |
| format.case_begin_required | cst (inline) | warning | `case` アイテムが複文なのに `begin/end` で括っていない場合を指摘。副作用をまとめて明示します。 |
| case.missing_default | cst (inline) | warning | `default` ブランチが無い `case` を検出。安全側の `default` を追加し、意図的に不要な場合も空ブランチを入れてください。 |
| lang.case_requires_unique | cst (inline) | warning | `unique`/`priority` の前置が無い `case` を指摘。競合を避けるため `unique case` などへ変更します。 |
| lang.no_delays | raw_text | warning | `#5` などの遅延構文を禁止。クロック同期構造に置き換え、遅延を RTL で表現しない方針を徹底します。 |
| lang.prefer_always_comb | raw_text | warning | `always @*` を検出し、`always_comb` への移行を促します。 |
| lang.no_always_latch | raw_text | warning | `always_latch` の使用を警告。FF ベースの記述へ改修し、意図しないラッチを防止します。 |
| lang.always_ff_reset | raw_text | warning | `always_ff` の感度リストに `negedge rst_n` が無い場合を指摘。非同期リセットを追加するか、設計ポリシーに応じてルールを無効化します。 |
| lang.always_comb_at | raw_text | warning | `always_comb` に `@(...)` が付いている場合を検出。感度リストを削除してください。 |

### 3.3 プロセス/代入ルール

| ルールID | ステージ | Severity | チェック内容と対処 |
| --- | --- | --- | --- |
| comb.nb_in_alwayscomb | cst (inline) | warning | `always_comb` 内の `<=` を検出。組合せロジックでは `=` を使い、ノンブロッキング代入を避けます。 |
| seq.blocking_in_alwaysff | cst (inline) | warning | `always_ff` 内の `=` を検知。順序ロジックでは `<=` を使用します。 |
| flow.multiple_nonblocking | ast | warning | 同一信号に対する複数のノンブロッキング代入を検出。単一 `always_ff` に統合するか、明示的な優先度制御へ整理します。 |

### 3.4 インタフェース/ヘッダー/パッケージ

| ルールID | ステージ | Severity | チェック内容と対処 |
| --- | --- | --- | --- |
| module.no_port_wildcard | raw_text / cst (inline) | warning | `.*` ワイルドカード接続を禁止。全ポートを `.port(signal)` 形式で書き出します（CST 版は構文解析、raw_text 版は文字列検査）。 |
| module.named_ports_required | raw_text | warning | 位置指定ポートを検出。命名ポート接続へ切り替えて可読性と保守性を確保します。 |
| global.local_define_undef | raw_text | warning | `_` で始まるローカルマクロが `` `undef`` されていない場合を警告。使用後に解除するか別ファイルへ限定します。 |
| global.prefer_parameters | raw_text | warning | グローバル `` `define`` を検知し、`parameter/localparam` への置換を推奨。共有定数はパッケージ内パラメータに集約します。 |
| header.missing_spdx | raw_text | warning | 先頭 200 文字に SPDX ヘッダーが無い場合を指摘。`// SPDX-License-Identifier: ...` を冒頭に追記します。 |
| header.missing_comment | raw_text | warning | 先頭 5 行に説明コメントが無い場合を警告。ファイル役割や依存関係を簡潔に記載します。 |
| package.multiple | raw_text | warning | 1 ファイルに複数 `package` が存在する場合を検出。ファイル分割または統合を検討します。 |
| package.missing_end | raw_text | warning | `endpackage` が無い場合を警告。閉じラベルを追加してください。 |
| package.end_mismatch | raw_text | warning | `endpackage : name` のラベル不一致を指摘。`package` 名と整合させます。 |
| package.define_in_package | raw_text | warning | `package` 内で `_` 以外の `` `define`` を検出。パラメータ化や `localparam` で表現します。 |

### 3.5 命名規約

| ルールID | ステージ | Severity | チェック内容と対処 |
| --- | --- | --- | --- |
| naming.module_case | ast | warning | モジュール名が lower_snake_case 以外の場合に警告。`foo_bar` 形式へ変更します。 |
| naming.signal_case | ast | warning | ネット/変数名の snake_case 違反を検出。テンプレートや自動生成スクリプトも含めて修正します。 |
| naming.port_case | ast | warning | ポート名の snake_case 違反を検出。 |
| naming.no_numeric_suffix | ast | warning | `_0` など末尾数字付き名称を禁止。配列や generate 文でステージを表現します。 |
| naming.suffix_order | ast | warning | `_n_i` など順序不正なサフィックスを指摘。`_ni/_no/_nio` へ揃えます。 |
| naming.clk_prefix | ast | warning | `clk` を含む名称が `clk` で始まらない場合を検出。クロックは `clk_*` に統一します。 |
| naming.rst_active_low | ast | warning | `rst` で始まる名称が `_n/_ni/_no/_nio` で終わらない場合を警告。能動 Low を後置で明示します。 |
| naming.clk_order | ast | warning | `clk` ポートがリセットや他ポートより後ろに移動した場合を指摘。ポート順序を `clk -> rst -> others` に保ちます。 |
| naming.rst_before_clk | ast | warning | リセットがクロックより前に定義されている場合を検出。クロックを先頭に移します。 |
| naming.differential_pair | ast | warning | `_p/_n` の差動ペア片側欠落を警告。対応する相方を追加してください。 |
| naming.pipeline_sequence | ast | warning | `_q2/_q3...` といった段数付き信号の前段不足を検出。欠けている段を実装するか命名を修正します。 |
| naming.parameter_upper | ast | warning | `parameter` 名が UpperCamelCase 以外の場合を指摘。`DataWidth` のように先頭大文字へ。 |

### 3.6 宣言・型・リテラル

| ルールID | ステージ | Severity | チェック内容と対処 |
| --- | --- | --- | --- |
| decl.unused.var | ast | warning | 読み書きが 0 回の変数を検出。不要なら削除し、将来利用予定ならコメントと TODO を残したうえでローカル設定で抑制を検討します。 |
| decl.unused.net | ast | warning | 参照されないネットを検出。配線整理やスタブ削除を行います。 |
| decl.unused.param | ast | warning | 未使用 `parameter/localparam` を指摘。使われていない定数は削除するか、`cfg` へ移すなど動機を明示します。 |
| typedef.enum_suffix | raw_text | warning | `typedef enum` 名が `_e` で終わらない場合を警告。 |
| typedef.type_suffix | raw_text | warning | その他の `typedef` 名が `_t` 終わりでない場合を指摘。 |
| width.unsized_base_literal | raw_text | warning | 幅なし基数リテラル（`'hFF` など）を検出。`8'hFF` のように明示的な幅を付けます。 |

### 3.7 補助ルール

| ルールID | ステージ | Severity | チェック内容と対処 |
| --- | --- | --- | --- |
| template.raw_text_marker | raw_text | info | `__SV_MINT_TEMPLATE__` というマーカー文字列を検出。テンプレート展開漏れを示すため、生成パスを確認します。 |
| debug.ping | 任意 | warning | ステージ名とシンボル個数を出すデバッグ用テンプレート。本番運用では `sv-mint.toml` の `ruleset.scripts` から除外してください。 |

## 4. 運用メモ
1. ルールを無効化したい場合は `sv-mint.toml` の `[ruleset] scripts` から該当プラグインを除外します。例: `format_line_length.py` を削除すると `format.line_length` が発火しなくなります。
2. Severity を調整する場合は `[ruleset.override]` に `rule_id = "error"` などを記述し、CI とローカルで同じ基準を保ちます。
3. チーム内で例外を一時許容する場合はローカル設定ファイルを別途用意し、共有リポジトリでは例外状態を残さない運用を心掛けてください。
