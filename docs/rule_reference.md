# ルールリファレンス

## 0. 想定読者と読み方
本書は sv-mint に標準で含まれる Python プラグイン群の仕様を一覧化したものです。設計/検証担当者がレポートを確認するときに「どのステージで何を見ているか」「どう修正すべきか」を素早く把握できるよう、以下を記載しています。
- **Rule ID**: CLI 出力に現れる ID。`ruleset.override` で指定するキーも同一です。
- **ステージ**: `raw_text` / `pp_text` / `cst` / `ast` のいずれか。payload の出どころを示します。
- **スクリプト**: `plugins/` 以下のファイル名。ルールを無効化したい場合は `ruleset.scripts` から除外します。
- **検出条件 / 対処のヒント**: 代表的なチェック内容と推奨修正方針。

## 1. ルール一覧（抜粋）
完全なリストは `plugins/` ディレクトリを参照してください。ここでは代表的なカテゴリを示します。

| 分類 | Rule ID 例 | ステージ | スクリプト | 概要 |
| --- | --- | --- | --- | --- |
| フォーマット | `format.line_length` / `format.no_tabs` | `raw_text` | `format_line_length.py` / `format_text_basics.py` / `format_spacing.py` / `format_indent_rules.py` | 行長・タブ・改行・マクロ周辺などテキスト整形。`format_text_basics` は ASCII 制約や最終改行も確認。 |
| 制御構造 | `format.begin_required` / `format.case_begin_required` | `cst` | `begin_end_required.py` / `format_case_begin_cst.py` | `if/else` や `case` の `begin ... end` 必須ルール、CST から抽出。 |
| 命名規約 | `naming.module_case`, `naming.parameter_upper` など | `ast` | `naming_rules.py` | モジュール/信号/typedef 名称のスタイルを AST から解析。 |
| 幅・リテラル | `width.unsized_base_literal` | `cst` | `width_literal_rules.py` | 幅無しリテラルや不正な進数指定を禁止。 |
| クロック/リセット | `lang.always_ff_reset`, `lang.prefer_always_comb` | `ast` | `lang_construct_rules.py` / `seq_blocking_in_alwaysff.py` / `comb_nb_in_alwayscomb.py` | `always_ff/comb` ブロック内の代入種別や reset 取り扱いを監視。 |
| リソース未使用 | `decl.unused.var` / `decl.unused.net` / `decl.unused.param` | `ast` | `decl_unused_*.py` | 宣言したものの参照されない変数/ネット/パラメータを検出。 |
| モジュール構造 | `module.named_ports_required`, `module.no_port_wildcard` | `cst` | `module_inst_rules.py` / `no_port_wildcard.py` | インスタンスでのワイルドカード禁止、named port を強制。 |
| グローバル定義 | `global.prefer_parameters` | `raw_text` | `global_define_rules.py` | `define` の乱用を検出し、`parameter` への置換を促す。 |
| パッケージ/typedef | `package.multiple`, `typedef.enum_suffix` | `ast` | `package_rules.py` / `typedef_naming_rules.py` | パッケージ内の重複や typedef の命名規則を確認。 |
| その他のテンプレート | `header.missing_spdx`, `debug.ping` | `raw_text` | `header_comment_rule.py` / `debug_ping.py` | SPDX ライセンス表記の不足やデバッグ用 ping ルール。 |

## 2. ステージ別の着目点
- **raw_text**: UTF-8 + LF 正規化済みの元ソース。コメント・マクロを含むため、行長・ASCII 制約・ヘッダー確認に最適。
- **pp_text**: `sv-parser` 前処理後のテキスト。`defines` 展開結果を見たい場合はこちらを利用。
- **cst**: `sv-parser` の構文木そのもの。ブロック境界やトークン種別を正確に取れるため、フォーマット/構文パターン系ルールに最適。
- **ast**: `AstSummary`（宣言/参照/シンボル/代入/ports 等）。命名規約や未使用検出など、セマンティクス指向のルールはこちらで実装します。

## 3. ルール個別メモ
- `format_line_length.py`: `MAX_COLUMNS = 100` を超える行に warning。`ruleset.override` で severity を変えても閾値は固定です。
- `format_text_basics.py`: ASCII/制御文字/最終改行など複数チェックをまとめています。メッセージ文言で対象が判別可能です。
- `lang_construct_rules.py`: `always_ff` や `case` に対する複数ルールを内包。Violation の `rule_id` ごとに個別 disable できます。
- `debug_ping.py`: payload の item 数を出すテンプレート。新規ルール開発時の疎通確認に活用してください。

## 4. ルールの有効・無効化
1. `sv-mint.toml` の `[ruleset] scripts` から対象ファイルを削除（無効化）または追加（有効化）。
2. CLI を再実行すると、常駐ホストが新しいスクリプト一覧で初期化されます。
3. ルールごとに severity だけ調整したい場合は `[ruleset.override]` を使用してください。

## 5. 既知制約
- 一部ルールは AST/シンボル解析に依存しているため、`ignore_include = true` かつ依存関係の解決ができない場合に誤検出する可能性があります。
- CST を用いるフォーマット系ルールは `sv-parser` のバージョンに依存します。`Cargo.toml` の `sv-parser` を更新した場合は `format_case_begin_cst.py` 等で再検証してください。

より詳しい payload 仕様は [docs/plugin_author.md](plugin_author.md) を、内部実装は [docs/internal_spec.md](internal_spec.md) を参照してください。
