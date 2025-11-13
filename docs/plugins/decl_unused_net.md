# decl_unused_net.py

- **対応スクリプト**: `plugins/decl_unused_net.py`
- **使用ステージ**: `ast`
- **主な入力フィールド**: `symbols`（`class == net` の参照情報）
- **提供ルール**:
  | Rule ID | Severity | 動作概要 |
  | --- | --- | --- |
  | `decl.unused.net` | warning | 宣言したネットが読み書きゼロ回のまま残っている場合に警告 |

## ルール詳細

### `decl.unused.net`
- **検出条件**: `symbols` の `class` が `net` で、`read_count` / `write_count` がともに 0 のものを抽出し、宣言位置で違反を生成します。
- **代表メッセージ**: `` unused net <module>.<name> ``
- **主な対処**: 未使用ネットを削除するか、将来利用予定のシンボルは `_unused` など黙認される名前へ変更します。
- **補足**: `sv-mint` の AST 集計はインクルード後の実体で動くため、条件付きコンパイルでのみ使用されるネットは `ignore_include` 設定に注意してください。
- **LowRISC 参照**: lowRISC SystemVerilog スタイルガイドでは「取り残された信号は読みにくさと誤解を生む」ため、未使用宣言を禁止しています。本ルールはその指針に従って不要ネットを排除します。
- **良い例**:

```systemverilog
logic req_i;
logic ack_o;
logic busy;  // 実際に読み書きされている
```

- **悪い例**:

```systemverilog
logic req_i;
logic ack_o;
logic debug_tap;  // どこからも参照されない
```

- **追加のポイント**: 自動生成コードで仮のネットを挿入している場合は、`/* unused */` などのコメントでは抑制できません。`sv-mint.toml` の `[[ruleset.allowlist]]` にパターンを追加するか、生成元で `unused_net_` のような命名規則へ寄せることで、後工程で一括除外できます。
