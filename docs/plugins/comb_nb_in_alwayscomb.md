# comb_nb_in_alwayscomb.py

- **対応スクリプト**: `plugins/comb_nb_in_alwayscomb.py`
- **使用ステージ**: `cst`
- **主な入力フィールド**: `cst_ir`（`tokens`, `tok_kind_table`, `line_starts`, `pp_text`）
- **提供ルール**:
  | Rule ID | Severity | 動作概要 |
  | --- | --- | --- |
  | `comb.nb_in_alwayscomb` | warning | `always_comb` ブロック内の非ブロッキング代入 (`<=`) を禁止 |

## ルール詳細

### `comb.nb_in_alwayscomb`
- **検出条件**: `AlwaysConstruct` のうち `always_comb` を特定し、そのトークン領域内で `<=`（`op_le`）が出現した位置を報告します。トークン情報が無い場合はテキスト走査にフォールバックします。
- **代表メッセージ**: `` nonblocking '<=' inside always_comb ``
- **主な対処**: 組み合わせ回路ではブロッキング代入 `=` へ変更し、状態保持が必要なら `always_ff` 等へ分離してください。
- **補足**: `sv-parser` のトークン種別が更新された場合は `tok_kind_table` に `op_le` が含まれていることを確認してください。
- **LowRISC 参照**: lowRISC SystemVerilog スタイルガイドは `always_comb` 内での `<=` を明確に禁止しており、`always_comb` は純粋な組み合わせ記述に限定することをルール化しています。
- **良い例**:

```systemverilog
always_comb begin
  result_d = a_i ^ b_i;  // ブロッキング代入のみ
end
```

- **悪い例**:

```systemverilog
always_comb begin
  result_q <= a_i ^ b_i;  // nonblocking を使用しておりコンボロジックの定義に反する
end
```

- **追加のポイント**: `always_comb` に `<=` が存在するとツールによってはフロップ推論を試みてしまいます。`macros` でオペレータを隠している場合も展開後に `<=` が現れると検出されるため、`ASSIGN(result_q, expr)` のようなヘルパーマクロを使用する場合は `=` と `<=` でマクロを分けておくと誤検出を避けられます。
