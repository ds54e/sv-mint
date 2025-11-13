# format_line_length.py

- **対応スクリプト**: `plugins/format_line_length.py`
- **使用ステージ**: `raw_text`
- **主な入力フィールド**: `text`（LF 正規化済みソース）
- **提供ルール**:
  | Rule ID | Severity | 動作概要 |
  | --- | --- | --- |
  | `format.line_length` | warning | 1 行 100 文字超を検出 |

## ルール詳細

### `format.line_length`
- **検出条件**: 行ごとに文字数を計測し、`MAX_COLUMNS = 100` を超える行で違反を生成します。列 101 以降を指す位置情報を返すため、違反箇所がすぐ把握できます。
- **代表メッセージ**: `` line exceeds 100 columns (118) ``
- **主な対処**: 長い式は一時変数へ切り出すか演算子基準で改行し、コメントも 100 文字以内に収めます。
- **補足**: 重大度は `ruleset.override` で変更可能ですが、しきい値はコード内で固定されています。
