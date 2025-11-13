# template_raw_text_rule.py

- **対応スクリプト**: `plugins/template_raw_text_rule.py`
- **使用ステージ**: `raw_text`
- **主な入力フィールド**: `text`
- **提供ルール**:
  | Rule ID | Severity | 動作概要 |
  | --- | --- | --- |
  | `template.raw_text_marker` | info | テンプレート用マーカー `__SV_MINT_TEMPLATE__` の存在を通知 |

## ルール詳細

### `template.raw_text_marker`
- **検出条件**: 生テキスト内で固定文字列 `__SV_MINT_TEMPLATE__` を検索し、最初に見つかった位置で情報レベルの違反を返します。
- **代表メッセージ**: `` template marker detected ``
- **主な対処**: テンプレートファイルから実プロジェクトへコピーした後はマーカーを削除し、誤検知を防ぎます。
- **補足**: ルールは 1 か所のみを報告します。複数マーカーが必要な場合はファイルごとにテンプレートを分けてください。
