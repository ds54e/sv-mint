# decl_unused_param.py

- **対応スクリプト**: `plugins/decl_unused_param.py`
- **使用ステージ**: `ast`
- **主な入力フィールド**: `symbols`（`class == param` の参照情報）
- **提供ルール**:
  | Rule ID | Severity | 動作概要 |
  | --- | --- | --- |
  | `decl.unused.param` | warning | 参照カウントが 0 のパラメータを検出 |

## ルール詳細

### `decl.unused.param`
- **検出条件**: `symbols` から `class == param` を選び、`ref_count`（無ければ `read_count`）が 0 のものを違反化します。
- **代表メッセージ**: `` unused param <module>.<name> ``
- **主な対処**: 未使用パラメータを削除、もしくは上位から渡される構成値であればモジュール内から参照されるように配線します。
- **補足**: 自動生成コードでダミーパラメータが許容されている場合は `ruleset.override` で Severity を下げる運用も可能です。
