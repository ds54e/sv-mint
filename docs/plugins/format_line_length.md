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
- **LowRISC 参照**: lowRISC スタイルガイドは SystemVerilog 行の上限を 100 文字に設定し、ドキュメントコメントも同じ制限に従うよう指示しています。
- **良い例**:

```systemverilog
assign req_mask = (req_q & enable_mask) | (pending_q & ~enable_mask);
```

- **悪い例**:

```systemverilog
assign req_mask = (req_q & enable_mask) | (pending_q & ~enable_mask) | (bypass_req_q & replay_mask & {WIDTH{guard_en_i}}); // 100 文字超
```

- **追加のポイント**: 自動生成コメントは容易に 100 文字を超えるため、生成スクリプト側で折り返し処理を入れておくと CI での再現性が高まります。`rustfmt.toml` の `max_width` とは独立している点にも注意してください。
