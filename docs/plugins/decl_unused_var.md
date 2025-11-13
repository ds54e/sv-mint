# decl_unused_var.py

- **対応スクリプト**: `plugins/decl_unused_var.py`
- **使用ステージ**: `ast`
- **主な入力フィールド**: `symbols`（`class == var`）
- **提供ルール**:
  | Rule ID | Severity | 動作概要 |
  | --- | --- | --- |
  | `decl.unused.var` | warning | 参照されない変数宣言を警告 |

## ルール詳細

### `decl.unused.var`
- **検出条件**: `symbols` の `read_count` と `write_count` がいずれも 0 の `var` を探索し、宣言位置で報告します。
- **代表メッセージ**: `` unused var <module>.<name> ``
- **主な対処**: 変数を削除するか、デバッグ用途なら `/* verilator lint_off UNUSED */` 等の抑止コメントを使う代わりにルール設定で無効化します。
- **補足**: 常に `sv-parser` が抽出した位置情報を返すため、宣言が展開元のインクルードファイルにある場合は `Location.file` も確認してください。
- **LowRISC 参照**: lowRISC スタイルガイドでは未使用変数をコードに残さないこと、どうしても必要なら `_unused` など明示的な名前を付けることを求めています。
- **良い例**:

```systemverilog
logic enable;
logic data_d;
logic data_q;

always_ff @(posedge clk_i) begin
  if (enable) data_q <= data_d;
end
```

- **悪い例**:

```systemverilog
logic enable;
logic data_d;
logic debug_shadow;  // 読み書きされない
```

- **追加のポイント**: `logic unused = 1'b0;` のように「未使用」であることを名前に含めれば、`sv-mint.toml` の `allowlist.regex = ".*_unused$"` で抑制できます。スコープ内で `unused` を束ねたい場合は `logic [3:0] spare_signals = '0;` としてまとめ、`spare_signals[0] = foo;` のようにデバッグフックとして活用することも検討してください。
