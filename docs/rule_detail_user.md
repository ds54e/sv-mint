# ルール詳細ガイド（ユーザー向け）

sv-mint が出力する SystemVerilog ルールを利用者目線で整理したドキュメントです。検証担当者が CLI の warning/error を見つけた際に、該当ルールの背景と対処方針を素早く確認できるようルール ID ごとに詳細をまとめています。チェックの実装詳細は `docs/rule_reference.md` と `plugins/*.py` を参照してください。

## ドキュメントの読み方
- **ステージ**列はルールがどの入力を見ているかを示します。`raw_text` は整形前のソース、`pp_text` はプリプロセス済み、`cst` は構文木（inline CST）、`ast` は宣言/参照情報です。
- **Severity** は既定値です。`sv-mint.toml` の `[ruleset.override]` で変更できます。
- **チェック内容と対処** には典型的な検出条件と修正のヒントを記載しています。ルールを一時無効化したい場合は `[ruleset] scripts` から対応プラグインを外してください。

## 1. フォーマット／テキスト整形

| ルールID | ステージ | Severity | チェック内容と対処 |
| --- | --- | --- | --- |
| format.line_length | raw_text | warning | 1 行が 100 文字を超えると指摘します。長い式は変数に切り出すか改行し、コメントも折り返してください。 |
| format.ascii_only | raw_text | warning | ASCII (0-127) 以外の文字を検出します。全角や制御文字を避け、必要ならエスケープシーケンスで表現します。 |
| format.no_tabs | raw_text | warning | タブ文字 `\t` を禁止します。2 or 4 スペースへ置換してください。 |
| format.no_trailing_whitespace | raw_text | warning | 行末にスペース/タブが残っている行を指摘します。エディタの trim 機能を有効にしてください。 |
| format.final_newline | raw_text | warning | ファイル末尾の改行が欠落していると警告します。POSIX 互換のため 1 行追加してください。 |
| format.indent_multiple_of_two | raw_text | warning | 先頭スペースが 2 の倍数でない行を検出します。自動フォーマッタで揃えるのが確実です。 |
| format.preproc_left_align | raw_text | warning | `\`define` や `\`ifdef` などのプリプロ指令を行頭以外で見つけた際に指摘します。インデントを削除して左寄せにしてください。 |
| format.line_continuation_right | raw_text | warning | バックスラッシュによる行継続記号の右側に不要な文字が無いか確認します。`\\` を行末に置いてください。 |
| format.comma_space | raw_text | warning | カンマ直後に空白が無い箇所を検出します。`, ` の形に統一します。 |
| format.call_spacing | raw_text | warning | 関数/タスク呼び出しの名前と `(` の間に空白が入った場合に警告します。呼び出し名と `(` を隣接させてください。 |
| format.macro_spacing | raw_text | warning | マクロ呼び出し `` `FOO (bar)`` などの余分な空白を検出します。`` `FOO(bar)`` に修正します。 |
| format.case_colon_spacing | cst (inline) | warning | `case` アイテムのラベルと `:` の直前に空白があると指摘します。`label:` の形に揃えてください。 |
| format.case_colon_after | cst (inline) | warning | `case` アイテムの `:` の直後に空白が無い場合に警告します。`label: stmt` のように 1 文字以上空けます。 |

## 2. 制御構造・case ブロック

| ルールID | ステージ | Severity | チェック内容と対処 |
| --- | --- | --- | --- |
| format.begin_required | pp_text | warning | `if/for/while/repeat/forever` の本体が複数行に渡るのに `begin/end` で囲まれていない場合に警告します。ブロックを `begin ... end` で明示してください。 |
| format.end_else_inline | pp_text | warning | `end` の次の行に `else` が置かれているケースを検出します。`end else` を同一行に配置して読みやすさを保ちます。 |
| format.case_begin_required | cst (inline) | warning | `case` アイテム内で複数ステートメントを `begin/end` で包んでいない箇所を指摘します。副作用をまとめるため `begin ... end` を追加します。 |
| case.missing_default | cst (inline) | warning | `case` 文に `default` ブランチが無いと警告します。安全側の `default` を追加し、不要なら明示的に `default: begin end` を入れます。 |
| lang.case_requires_unique | cst (inline) | warning | `unique` または `priority` が前置されていない `case` を検出します。条件競合を避けたい場合は `unique case` などへ書き換えてください。 |
| lang.no_delays | raw_text | warning | `#5` などの遅延記述を禁止します。サイクル正確なロジックにはクロック同期構造を使用します。 |
| lang.prefer_always_comb | raw_text | warning | `always @*` を `always_comb` へ移行することを促します。 |
| lang.no_always_latch | raw_text | warning | `always_latch` の使用を検出します。明示的な FF に置き換え、ラッチを避けてください。 |
| lang.always_ff_reset | raw_text | warning | `always_ff` の感度リストに `negedge rst_n` が含まれていない場合に警告します。非同期リセットを追加するか、ルールを無効化する方針を決めてください。 |
| lang.always_comb_at | raw_text | warning | `always_comb` に感度リストが付いているときに警告します。`@(...)` を削除します。 |

## 3. プロセスと代入

| ルールID | ステージ | Severity | チェック内容と対処 |
| --- | --- | --- | --- |
| comb.nb_in_alwayscomb | cst (inline) | warning | `always_comb` ブロック内のノンブロッキング代入（`<=`）を検出します。組合せロジックでは `=` に置き換えてください。 |
| seq.blocking_in_alwaysff | cst (inline) | warning | `always_ff` ブロック内でブロッキング代入（`=`）を使っていると警告します。フリップフロップには `<=` を使用します。 |
| flow.multiple_nonblocking | ast | warning | 同一信号に対する複数のノンブロッキング代入を検出します。共通の `always_ff` へ統合するか、優先順位を明確にしてください。 |

## 4. モジュール／ヘッダー／パッケージ

| ルールID | ステージ | Severity | チェック内容と対処 |
| --- | --- | --- | --- |
| module.no_port_wildcard | raw_text / cst (inline) | warning | `.*` ワイルドカード接続を禁止します。各ポートを `.port(signal)` 形式で記述してください。CST ルールは構文木レベルでの検出、raw_text ルールはテキスト検索です。 |
| module.named_ports_required | raw_text | warning | モジュールインスタンスでの位置指定ポートを検出します。命名ポート接続へ書き換えて可読性を高めます。 |
| global.local_define_undef | raw_text | warning | `_` で始まるローカルマクロが `\`undef` されていない場合に指摘します。使用後に `\`undef` を追加するか、スコープを限定してください。 |
| global.prefer_parameters | raw_text | warning | グローバルマクロ使用を検出し、`parameter` への置換を推奨します。共有定数はパッケージ内パラメータで定義します。 |
| header.missing_spdx | raw_text | warning | ファイル冒頭 200 文字に SPDX ヘッダーが無い場合に警告します。`// SPDX-License-Identifier: ...` を追加してください。 |
| header.missing_comment | raw_text | warning | 先頭 5 行に説明コメントが無い場合に警告します。役割や依存関係を 1-2 行で記述します。 |
| package.multiple | raw_text | warning | 1 ファイルに複数の `package` 宣言があると指摘します。ファイルを分割するか `package` を統合します。 |
| package.missing_end | raw_text | warning | `package` に対応する `endpackage` が無い場合に警告します。閉じラベルを追加してください。 |
| package.end_mismatch | raw_text | warning | `endpackage : name` のラベルが `package name` と一致しない場合に指摘します。名称を揃えます。 |
| package.define_in_package | raw_text | warning | `package` 内で `_` 以外で始まる `\`define` を使用していると警告します。パラメータ化や `localparam` へ移行してください。 |

## 5. 命名規約

| ルールID | ステージ | Severity | チェック内容と対処 |
| --- | --- | --- | --- |
| naming.module_case | ast | warning | モジュール名が lower_snake_case 以外の場合に警告します。`foo_bar` 形式へ変更します。 |
| naming.signal_case | ast | warning | ネット/変数名の snake_case 違反を検出します。 |
| naming.port_case | ast | warning | ポート名の snake_case 違反を検出します。生成時のテンプレートを修正してください。 |
| naming.no_numeric_suffix | ast | warning | `_0` などの末尾数字付きシグナル名を禁止します。配列やインデックスで段階を表現します。 |
| naming.suffix_order | ast | warning | `_n_i` など順序不正なサフィックスを検出します。`_ni/_no/_nio` へ揃えます。 |
| naming.clk_prefix | ast | warning | `clk` を含む信号が `clk` で始まっていない場合に指摘します。クロックは `clk_*` 表記に統一します。 |
| naming.rst_active_low | ast | warning | `rst` で始まる信号が `_n/_ni/_no/_nio` で終わっていない場合に警告します。能動 Low を明示してください。 |
| naming.clk_order | ast | warning | `clk` ポートの後に別の `clk` が出現したり、`rst` より後に移動している場合に警告します。ポートリストの並び順を `clk -> rst -> その他` に保ちます。 |
| naming.rst_before_clk | ast | warning | リセットがクロックより前に定義されている場合に指摘します。クロックを先頭に移動します。 |
| naming.differential_pair | ast | warning | `_p/_n` の差動ポートが片方欠けている場合に警告します。対になる信号を追加してください。 |
| naming.pipeline_sequence | ast | warning | `_q2/_q3...` などの段数付き信号で前段が存在しない場合に指摘します。欠けているステージを実装するか命名を修正します。 |
| naming.parameter_upper | ast | warning | `parameter` 名が UpperCamelCase でない場合に警告します。共有定数 `DataWidth` のように先頭大文字へ変更します。 |

## 6. 宣言・型・リテラル

| ルールID | ステージ | Severity | チェック内容と対処 |
| --- | --- | --- | --- |
| decl.unused.var | ast | warning | 読み書き回数が 0 の `logic/reg` などを検出します。削除するか使用箇所を追加してください。 |
| decl.unused.net | ast | warning | 参照されないネットを検出します。宣言整理を促します。 |
| decl.unused.param | ast | warning | 参照されない `parameter/localparam` を指摘します。不要なら削除し、将来用フラグには TODO を残してから抑制を検討します。 |
| typedef.enum_suffix | raw_text | warning | `typedef enum` の型名が `_e` で終わらない場合に警告します。 |
| typedef.type_suffix | raw_text | warning | その他の `typedef` 名が `_t` で終わらない場合に警告します。 |
| width.unsized_base_literal | raw_text | warning | `’hFF` など幅を指定しない基数リテラルを検出します。`8'hFF` のように明示的なビット幅を追加してください。 |

## 7. 補助ルール

| ルールID | ステージ | Severity | チェック内容と対処 |
| --- | --- | --- | --- |
| template.raw_text_marker | raw_text | info | `__SV_MINT_TEMPLATE__` というプレースホルダー文字列を見つけるためのテンプレートルールです。テンプレート展開漏れを検出したらファイル生成手順を確認します。 |
| debug.ping | 任意 | warning | デバッグ用にステージ名とシンボル数を出力します。本番では `sv-mint.toml` の `ruleset.scripts` から外してください。 |

## ルールのカスタマイズ手順
1. `sv-mint.toml` の `[ruleset]` セクションで不要なプラグインをコメントアウト／削除します。例: `format_line_length.py` を除外すると `format.line_length` が無効化されます。
2. 個別の重大度を変える場合は `[ruleset.override]` に `rule_id = "error"` のように記述します。ここに載っている `rule_id` を正確に指定してください。
3. 許容したい一時的な例外には `// sv-mint:disable rule_id` のようなメタコメントはありません。どうしても必要であればローカル設定ファイルを分け、共有リポジトリではルール逸脱を残さない運用にしてください。
