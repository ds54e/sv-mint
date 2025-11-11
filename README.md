
sv-mint v2.7 仕様書
================================

1. 目的と適用範囲
----------------
sv-mint は SystemVerilog の静的チェックを行うツールで、Rust 製コアが sv-parser により前処理・構文解析を行い、解析成果物を Python プラグインへ段階的に渡してルール評価を行います。本仕様は v2.7 の現在実装に基づく動作要件、入出力、ステージ構成、ルール定義、ログ、終了コード、制限事項を規定します。

2. 用語
-------
- 入力ファイル: 解析対象の SystemVerilog ファイル
- ステージ: raw_text, pp_text, cst, ast から成る処理段階
- プラグイン: 各ステージのペイロードを受け取り診断配列を返す Python スクリプト
- 診断: code, severity, message, loc を含む1件の検出結果

3. 対象環境
-----------
- 対応 OS: Windows 10 以降
- Rust: stable MSVC toolchain
- Python: py -3 -u で起動される CPython 3.x
- 文字コード: UTF-8（BOM 可）
- 改行: CRLF/CR/LF を許容。内部表現は LF に正規化

4. CLI
------
書式
```
sv-mint [--config <path-to-toml>] <input>...
```
要件
- input は1個以上のファイルパス
- ディレクトリやワイルドカードは対象外
- 拡張子は不問
- 終了コード
  - 0: 正常（違反なし）
  - 2: 違反あり（診断が1件以上）
  - 3: 異常終了（入力不正、プラグイン異常、タイムアウトなど）

5. 設定ファイル（TOML）
-----------------------
トップレベル
```
[defaults]
timeout_ms_per_file = 3000

[plugin]
cmd = "py"
args = ["-3","-u","plugins/rules.py"]

[stages]
enabled = ["raw_text","pp_text","cst","ast"]

[svparser]
include_paths = []
defines = []
strip_comments = true
ignore_include = false
allow_incomplete = true

[rules]

[logging]
level = "info"
stderr_snippet_bytes = 2048
show_stage_events = true
show_plugin_events = true
show_parse_events = true
```
要件
- Windows で TOML 文字列中に絶対パスを埋め込む場合は、バックスラッシュを `\\` にエスケープすること
- 設定ファイルが存在しない場合の取り扱いは実装に依存。本仕様は上記の既定を推奨とする

6. ステージとペイロード
----------------------
パイプライン順序
```
raw_text → pp_text → cst → ast
```

6.1 raw_text
- 内容: BOM 除去・改行正規化済みの入力テキスト
- 目的: プラグインが生テキストを扱う検査を行う

6.2 pp_text
- 内容: 前処理後テキストとマクロ定義
- 目的: マクロ展開後の文字列を用いた検査

6.3 cst
- 内容: 構文木存在フラグなど最小メタ情報
- 目的: パース可否や構文木有無による検査

6.4 ast
- 内容: 宣言 decls、参照 refs、代入 assigns、シンボルテーブル symbols
- 目的: ルール評価（未使用・書き込みのみ・幅不一致など）

7. プラグインインターフェース
-----------------------------
- 起動: 設定の plugin.cmd と plugin.args に従い各ステージごとに1回起動
- 入力: 標準入力で1個の JSON オブジェクト（ステージのペイロード）
- 出力: 標準出力で JSON 配列（診断の配列）
- 要件
  - 未対応ステージや診断なしの場合は空配列 `[]` を返す
  - 出力は1行の完全な JSON とし、部分出力や空文字列を避ける
  - 標準エラー出力はログされ、一定バイト数で切り詰められる

8. AST ペイロード詳細
--------------------
8.1 decls
- 形式: 配列。各要素は以下のキーを持つ
  - name: 文字列
  - kind: 文字列（parameter, localparam, net, variable など）
  - range/packed_range/bit_range: オブジェクト（msb/lsb などの式文字列を含みうる）
  - width: 整数（あれば優先）
  - loc: { path, line, col }

8.2 refs
- 形式: 配列。各要素は以下のキーを持つ
  - name: 文字列
  - module: 文字列（所属モジュール名）
  - rw: 文字列（"read" または "write"）
  - loc: { path, line, col }

8.3 assigns
- 形式: 配列。各要素は以下のキーを持つ
  - kind: 文字列（blocking, nonblocking, continuous）
  - lhs: 式オブジェクト
  - rhs: 式オブジェクト
  - loc: { path, line, col }
- 式オブジェクトは少なくとも text（式文字列）および loc を持つ

8.4 symbols
- 形式: オブジェクトまたは配列。名前から既定値や式文字列へ解決可能な構造を持つ
- 用途: パラメータやローカルパラメータの値解決

9. 診断フォーマット
-------------------
- プラグイン出力の各要素
  - code: 文字列（例: "decl.unused", "var.writeonly", "width.mismatch"）
  - severity: 文字列（"warning" を標準とする）
  - message: 文字列（追加情報）
  - loc: { path, line, col }
- コアの表示形式
  - `path:line:col: [severity] code: message`

10. ルール定義
--------------
10.1 decl.unused
- 対象: parameter, localparam, net, variable などの宣言
- 判定: 参照集合に当該識別子の read/write が一度も現れない場合に報告
- 例外: ビット幅レンジや定数式における識別子の使用は read として扱う

10.2 var.writeonly
- 対象: net, variable
- 判定: write が存在し、read が一度も現れない場合に報告

10.3 width.mismatch
- 対象: assigns の各代入
- 判定: LHS と RHS のビット幅が一致しない場合に報告
- 幅評価規則
  - 宣言に width がある場合はそれを採用
  - range の msb/lsb から `abs(msb-lsb)+1` を算出
  - 式評価は定数、名前、単項演算（+,-,~）、二項演算（+,-,*,//,<<,>>,|,&,^）を許容
  - 整数値のビット幅は `value.bit_length()`（0 は 1 とする）
  - 評価不能な場合は未定義とみなす

11. ログ
--------
- 出力先: 標準エラー
- イベント: parse_preprocess_start/done、parse_parse_start/done、parse_ast_collect_done、stage_start/done、plugin_invoke/done 等
- 出力制御: logging.show_stage_events、logging.show_plugin_events、logging.show_parse_events により制御
- 標準エラー取り込みは stderr_snippet_bytes を上限として切り詰め

12. タイムアウトと上限
----------------------
- 1ファイル当たりのプラグイン実行時間上限は defaults.timeout_ms_per_file
- 出力サイズやエラー出力は実装側の上限でカットされる

13. 終了コード
--------------
- 0: 診断 0 件
- 2: 診断 1 件以上
- 3: 入力不正、プラグイン異常、タイムアウトなど

14. 例
------
入力
```
module top #(parameter W=8) ();
  logic [W-1:0] y;
  logic [3:0]   x;
  assign y = x;
endmodule
```
期待される診断
```
[path]:[line]:[col]: [warning] var.writeonly: 'y' written but never read
[path]:[line]:[col]: [warning] width.mismatch: lhs=8, rhs=4
```

15. 既知の制限と注意事項
------------------------
- 複雑な式、連接、部分選択、キャストの完全評価は将来拡張
- Windows の TOML 文字列中に絶対パスを埋め込む際は `\\` によるエスケープが必要
- プラグインは未対応ステージで必ず空配列を返すこと
