# sv-mint

## 概要
sv-mint は SystemVerilog の静的検査ツールです。Rust 製コアが sv-parser により前処理と構文解析を実行し、各ステージの成果物を Python プラグインへ渡して診断を得ます。本書は v2.7 の仕様と現行実装に基づき、導入から運用までの手順とデータ仕様を記載します。

## 対象環境
- 対応 OS は Windows 10 以降です。
- Rust は stable の MSVC ツールチェーンを使用します。
- Python は CPython 3 系を使用し、起動は py -3 -u を前提とします。
- 文字コードは UTF-8 を前提とします。BOM を許容します。
- 改行コードは CRLF と LF を許容します。内部では LF に正規化します。

## ビルドとセットアップ
Rust ツールチェーンを導入後、次の手順でビルドします。

```
cargo build --release
```

生成物の例

```
target\release\sv-mint.exe
```

プラグインは Python スクリプトを想定します。仮の配置例は次のとおりです。

```
plugins\rules.py
```

## 使い方
### コマンド書式
```
sv-mint [--config <path-to-toml>] <input>...
```

### 注意事項
- input は 1 個以上のファイルパスです。
- ディレクトリやワイルドカードは対象外です。
- 拡張子は問いません。

## 終了コード
- 0 は診断が 0 件の場合です。
- 2 は診断が 1 件以上存在する場合です。
- 3 は入力不正やプラグイン異常、タイムアウトなどの異常終了です。

## 設定ファイル
TOML で記述します。例を示します。

```toml
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

### 注意事項
Windows で TOML に絶対パスを埋め込む場合はバックスラッシュを二重にします。

## パイプライン
ステージの順序は次のとおりです。

```
raw_text → pp_text → cst → ast
```

### 各ステージの目的
- raw_text は BOM 除去と改行正規化を行った入力テキストを扱います。
- pp_text は前処理後のテキストとマクロ定義を扱います。
- cst は構文木の有無など最小限のメタ情報を扱います。
- ast は宣言、参照、代入、シンボルテーブルを扱います。

## プラグインインターフェース
### 起動
設定の plugin.cmd と plugin.args に従い、各ステージごとに 1 回起動します。

### 入力
標準入力で 1 個の JSON オブジェクトを受け取ります。

### 出力
- 標準出力で JSON 配列を 1 行で出力します。
- 未対応ステージや診断がない場合は空配列を返します。
- 標準エラーはログに取り込まれ、指定バイト数で切り詰められます。

### プラグインの最小骨子例
```python
import sys, json

def main():
    payload = json.loads(sys.stdin.read())
    result = []
    print(json.dumps(result))

if __name__ == "__main__":
    main()
```

## 診断フォーマット
プラグインは次の形式の配列を返します。

```
[
  {
    "code": "decl.unused",
    "severity": "warning",
    "message": "detail",
    "loc": {"path":"x.sv","line":1,"col":1}
  }
]
```

## AST ペイロード仕様
### decls
- name は識別子名です。
- kind は parameter、localparam、net、variable などの分類です。
- range、packed_range、bit_range はレンジ情報を表します。
- width は整数です。存在する場合は優先します。
- loc は path、line、col を持ちます。

### refs
- name、module、rw、loc を持ちます。rw は read または write です。

### assigns
- kind は blocking、nonblocking、continuous のいずれかです。
- lhs と rhs は式オブジェクトです。少なくとも text と loc を持ちます。
- loc は path、line、col を持ちます。

### symbols
名前から既定値や式を解決できる構造を持ちます。

## 実装済みルール
### decl.unused
宣言が参照集合に一度も現れない場合に報告します。

### var.writeonly
書き込みが存在して読み出しが一度もない場合に報告します。

### width.mismatch
代入において左辺と右辺のビット幅が一致しない場合に報告します。

#### 幅評価の規則
- 宣言に width がある場合はそれを採用します。
- range の msb と lsb から絶対値の差に 1 を加えた値を用います。
- 定数、名前、単項演算と二項演算の一部を評価対象とします。
- 整数値のビット幅は値のビット長を用い、ゼロは 1 とします。
- 評価不能な式は未定義とみなします。

## ログ出力
- 出力先は標準エラーです。
- parse_preprocess_start、parse_parse_start、parse_ast_collect_done、stage_start、plugin_invoke などのイベントを記録します。
- logging.show_stage_events、logging.show_plugin_events、logging.show_parse_events により制御します。
- 標準エラー取り込みは stderr_snippet_bytes を上限として切り詰めます。

## タイムアウトと上限
- 1 ファイル当たりのプラグイン実行時間の上限は defaults.timeout_ms_per_file です。
- 出力やエラーの取り込みは実装側の上限で切り詰められます。

## 使用例
### 入力
```systemverilog
module top #(parameter W=8) ();
  logic [W-1:0] y;
  logic [3:0]   x;
  assign y = x;
endmodule
```

### 想定される診断
```
[path]:[line]:[col]: [warning] var.writeonly: 'y' written but never read
[path]:[line]:[col]: [warning] width.mismatch: lhs=8, rhs=4
```

## 既知の制限
- 複雑な式、連接、部分選択、キャストの完全評価は未対応です。
- 未対応ステージではプラグインは空配列を返します。
- Windows の TOML に絶対パスを埋め込む場合はバックスラッシュを二重にします。

## 仕様と実装の既知の差分
- assigns の分類は仕様では kind を使用します。実装では演算子表現を使用する箇所が存在します。
- decls の分類語は仕様では variable を示します。実装では var を使用する箇所が存在します。
- width.mismatch のメッセージ表記は仕様の例ではカンマ区切りです。実装では対比表現を使用する箇所が存在します。

## トラブルシューティング
- 終了コードが 3 の場合は入力不正、プラグイン異常、タイムアウトの可能性があります。
- プラグインが空行や部分的な JSON を出力すると解析に失敗します。1 行の完全な JSON を出力してください。
- plugin.cmd と args の設定不整合に注意してください。

## 生成情報
本ソフトウェアおよび本ドキュメントは ChatGPT により作成されています。
