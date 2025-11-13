# header_comment_rule.py

- **対応スクリプト**: `plugins/header_comment_rule.py`
- **使用ステージ**: `raw_text`
- **主な入力フィールド**: `text`
- **提供ルール**:
  | Rule ID | Severity | 動作概要 |
  | --- | --- | --- |
  | `header.missing_spdx` | warning | ファイル先頭 200 文字に SPDX 表記が無い場合に警告 |
  | `header.missing_comment` | warning | 先頭 5 行以内に行コメントが無い場合にヘッダー不足を通知 |

## ルール詳細

### `header.missing_spdx`
- **検出条件**: 先頭 200 文字から `SPDX-License-Identifier` を検索し、見つからなければ行頭位置で報告します。
- **代表メッセージ**: `` file should include SPDX-License-Identifier header ``
- **主な対処**: 1 行目付近に `// SPDX-License-Identifier: Apache-2.0` のような宣言を追記します。

### `header.missing_comment`
- **検出条件**: ファイル冒頭 5 行に `//` から始まるコメントが存在しない場合に違反を生成します。
- **代表メッセージ**: `` file header should include descriptive comment ``
- **主な対処**: モジュール用途や連絡先などの概要コメントを追加してコンテキストを明示します。
