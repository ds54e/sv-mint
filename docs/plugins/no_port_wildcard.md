# no_port_wildcard.py

- **対応スクリプト**: `plugins/no_port_wildcard.py`
- **使用ステージ**: `cst`（`mode = inline`）
- **主な入力フィールド**: `cst_ir.tokens`, `tok_kind_table`, `line_starts`
- **提供ルール**:
  | Rule ID | Severity | 動作概要 |
  | --- | --- | --- |
  | `module.no_port_wildcard` | warning | CST レベルでの `.*` ポート接続を禁止 |

## ルール詳細

### `module.no_port_wildcard`
- **検出条件**: `conn_wildcard` トークンをそのまま抽出し、正確な位置（ファイル・行・列）で報告します。
- **代表メッセージ**: `` named port connections must not use .* wildcard ``
- **主な対処**: 手動で `.port(signal)` へ展開するか、自動生成スクリプトを修正します。
- **補足**: `module_inst_rules.py` でも同様のチェックがありますが、こちらは CST に依存するためプリプロ展開後でも精度高く検出できます。
- **LowRISC 参照**: lowRISC でも `.*` の使用は禁止されており、全ポートを明示的に列挙することが義務付けられています。
- **良い例**:

```systemverilog
submod u_sub (
  .clk_i (clk_i),
  .rst_ni(rst_ni)
);
```

- **悪い例**:

```systemverilog
submod u_sub (.*);
```

- **追加のポイント**: CST ベースの検出なのでマクロ展開後に `.*` が現れても検知可能です。`generate` ブロック内で `.*` を使っているコードを修正する際は、`sv-vip` などの自動配線ツールに任せるのが安全です。
