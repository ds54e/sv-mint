# width_literal_rules.py

- **対応スクリプト**: `plugins/width_literal_rules.py`
- **使用ステージ**: `raw_text`
- **主な入力フィールド**: `text`
- **提供ルール**:
  | Rule ID | Severity | 動作概要 |
  | --- | --- | --- |
  | `width.unsized_base_literal` | warning | `'hFF` など幅未指定の基数リテラルを禁止 |

## ルール詳細

### `width.unsized_base_literal`
- **検出条件**: 正規表現 `(?<![0-9_])'(b|B|d|D|h|H|o|O)` で幅無しの基数リテラルを検出し、位置を報告します。
- **代表メッセージ**: `` base literal must include explicit width (e.g. 8'hFF) ``
- **主な対処**: すべての基数付きリテラルに `8'h`, `4'd` のようなビット幅を追加してください。
- **補足**: ブール演算にマッチさせる追加の正規表現は現状コメントアウトされています。必要なら拡張を検討してください。
