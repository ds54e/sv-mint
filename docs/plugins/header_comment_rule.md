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
- **LowRISC 参照**: lowRISC プロジェクト（OpenTitan 含む）は全ファイルに SPDX ヘッダーを付けることを必須としており、Apache-2.0 と Proprietary の切り替えも SPDX ラベルで行います。
- **良い例**:

```systemverilog
// SPDX-License-Identifier: Apache-2.0
// DMA channel control logic
```

- **悪い例**:

```systemverilog
// DMA channel control logic  // SPDX が無い
```

- **追加のポイント**: 生成ファイルで SPDX を付けるのが難しい場合は、生成スクリプト側でテンプレートに組み込むと抜け漏れを防げます。

### `header.missing_comment`
- **検出条件**: ファイル冒頭 5 行に `//` から始まるコメントが存在しない場合に違反を生成します。
- **代表メッセージ**: `` file header should include descriptive comment ``
- **主な対処**: モジュール用途や連絡先などの概要コメントを追加してコンテキストを明示します。
- **LowRISC 参照**: lowRISC スタイルガイドはファイル先頭で設計の役割やライセンスを説明するコメントを書くよう求めています。
- **良い例**:

```systemverilog
// Control logic for entropy distribution network.
module entropy_ctrl (...);
```

- **悪い例**:

```systemverilog
module entropy_ctrl (...);
```

- **追加のポイント**: コメントにはモジュール名、役割、依存関係の URL などを含めるとレビュー時に意図を把握しやすくなります。CI で自動生成する際もテンプレートに説明文を入れてください。
