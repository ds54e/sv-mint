# module_inst_rules.py

- **対応スクリプト**: `plugins/module_inst_rules.py`
- **使用ステージ**: `raw_text`
- **主な入力フィールド**: `text`
- **提供ルール**:
  | Rule ID | Severity | 動作概要 |
  | --- | --- | --- |
  | `module.no_port_wildcard` | warning | `.*` ワイルドカードによるポート結線を禁止 |
  | `module.named_ports_required` | warning | 位置指定ポートを禁止し、`.port(signal)` 形式を強制 |

## ルール詳細

### `module.no_port_wildcard`
- **検出条件**: 正規表現 `\.\*` で `.*` 接続を検知し、使用箇所の位置を報告します。
- **代表メッセージ**: `` avoid .* wildcard in module instantiations ``
- **主な対処**: 接続漏れを防ぐため、明示的な named port 接続に変換してください。
- **LowRISC 参照**: lowRISC スタイルガイドは `.*` を禁止し、すべてのポートを明示的に列挙することを求めています。
- **良い例**:

```systemverilog
foo u_foo (
  .clk_i (clk_i),
  .rst_ni(rst_ni),
  .req_i (req_i)
);
```

- **悪い例**:

```systemverilog
foo u_foo (.*);
```

- **追加のポイント**: `.*` はモジュール更新時に新規ポートが自動結線されてしまうため、シミュレーションと合成での意図しない挙動につながります。

### `module.named_ports_required`
- **検出条件**: 位置指定で始まるインスタンス（行頭が `foo bar (` で `.` に続かない）を検知します。
- **代表メッセージ**: `` use named port connections instead of positional arguments ``
- **主な対処**: `.clk(clk)` のような named port へ書き換え、並び替え時のリスクを無くします。
- **LowRISC 参照**: lowRISC では named port 以外を禁止し、順序依存の結線を避けるよう規定しています。
- **良い例**:

```systemverilog
bar u_bar (
  .clk_i(clk_i),
  .ready_o(ready_o)
);
```

- **悪い例**:

```systemverilog
bar u_bar (clk_i, ready_o);
```

- **追加のポイント**: 自動修正時は `verible-verilog-format --named-port-formatting` 等を併用すると整形が容易です。`.*` 抑止ルールと併せて使うと結線ポリシーを統一できます。
