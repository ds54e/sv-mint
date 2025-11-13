# debug_ping.py

- **対応スクリプト**: `plugins/debug_ping.py`
- **使用ステージ**: 任意（呼び出し時の `stage` をそのまま表示）
- **主な入力フィールド**: `payload.ast` / `payload.symbols` など AST 由来の統計
- **提供ルール**:
  | Rule ID | Severity | 動作概要 |
  | --- | --- | --- |
  | `debug.ping` | warning | 受け取ったステージ名とシンボル数をエコーバックし、パイプライン疎通を確認 |

## ルール詳細

### `debug.ping`
- **検出条件**: 常に 1 件の違反を生成し、`payload` 内で見つかった `symbols`（もしくは `ast.symbols`）の件数をメッセージに含めます。
- **代表メッセージ**: `` debug ping: stage=ast, symbols=42 ``
- **主な対処**: デバッグ専用なので本番ルールセットでは `sv-mint.toml` の `[ruleset.scripts]` から削除します。
- **補足**: ルール拡張時にデータが想定どおり届いているかを素早く確認する用途を想定しています。Severity は `to_viol` 呼び出し時に上書き可能です。
- **LowRISC 参照**: lowRISC のスタイルガイドに直接対応する規定はありませんが、デバッグ用コードやルールはリリースビルドに混入させないという一般原則に沿う形で利用します。
- **良い例**（開発中のみ有効化）:

```toml
[ruleset.scripts]
debug_ping = { path = "plugins/debug_ping.py", stage = "ast", enabled = true }

[profile.release.ruleset.scripts]
debug_ping = { enabled = false }
```

- **悪い例**（本番設定で ping を残したまま）:

```toml
[ruleset.scripts]
debug_ping = { path = "plugins/debug_ping.py", stage = "ast" }
# release でも無効化せず、常時 warning を発生させてしまう
```

- **追加のポイント**: CI で `debug.ping` が報告されると他の重要な診断が埋もれるため、PR 用ルールセットでは確実に外しておきます。別の統計値を見たい場合は `payload` から自由にメッセージへ追加できるため、拡張テスト時のログ計測に活用できます。
