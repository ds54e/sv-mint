# typedef_naming_rules.py

- **対応スクリプト**: `plugins/typedef_naming_rules.py`
- **使用ステージ**: `raw_text`
- **主な入力フィールド**: `text`
- **提供ルール**:
  | Rule ID | Severity | 動作概要 |
  | --- | --- | --- |
  | `typedef.enum_suffix` | warning | `typedef enum` 名の末尾を `_e` に統一 |
  | `typedef.type_suffix` | warning | その他の `typedef` 名を `_t` で終わらせる |

## ルール詳細

### `typedef.enum_suffix`
- **検出条件**: `typedef enum { ... } name;` の `name` が `_e` で終わっていない場合に警告。
- **代表メッセージ**: `` enum types should end with _e: state ``
- **主な対処**: `state_e` のように末尾を `_e` に変更します。

### `typedef.type_suffix`
- **検出条件**: 非 enum の `typedef` 名が `_t` で終わらない場合に報告。
- **代表メッセージ**: `` typedef names should end with _t: data ``
- **主な対処**: `data_t` のように `_t` サフィックスを付与します。
