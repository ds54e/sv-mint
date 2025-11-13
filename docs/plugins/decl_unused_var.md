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
