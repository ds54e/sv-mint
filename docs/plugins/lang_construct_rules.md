# lang_construct_rules.py

- **対応スクリプト**: `plugins/lang_construct_rules.py`
- **使用ステージ**: `raw_text`
- **主な入力フィールド**: `text`
- **提供ルール**:
  | Rule ID | Severity | 動作概要 |
  | --- | --- | --- |
  | `lang.no_delays` | warning | `#5` などの遅延構文を禁止 |
  | `lang.prefer_always_comb` | warning | `always @*` を `always_comb` へ置換することを要求 |
  | `lang.no_always_latch` | warning | `always_latch` の使用を警告 |
  | `lang.always_ff_reset` | warning | `always_ff` の感度に `negedge rst_n` が無い場合を指摘 |
  | `lang.always_comb_at` | warning | `always_comb` に感度リスト `@(...)` を持たせない |

## ルール詳細

### `lang.no_delays`
- **検出条件**: `#` 記号に続く即値遅延を検索し、`#( ... )` 以外のパターンで遅延を検出します。
- **代表メッセージ**: `` delay (#) constructs are not permitted ``
- **主な対処**: タイミングを RTL では表現せず、テストベンチや SDC へ移してください。
- **LowRISC 参照**: lowRISC スタイルガイドは合成可能コードでの `#delay` を禁止し、タイミング制約は STA に任せるよう定義しています。
- **良い例**:

```systemverilog
assign valid_o = req_i & ready_i;
```

- **悪い例**:

```systemverilog
#5 valid_o = req_i;  // 遅延を RTL に埋め込んでいる
```

- **追加のポイント**: `#(DEPTH)` のようなモジュールパラメータ宣言は対象外です。`#0` でも同様に禁止されるため、不要なデルタサイクルを入れないようにしてください。

### `lang.prefer_always_comb`
- **検出条件**: `always @*` を検出して `always_comb` への置換を促します。
- **代表メッセージ**: `` use always_comb instead of always @* ``
- **主な対処**: 感度リストを書かずに `always_comb` を使用し、ツールの等価性を保ちます。
- **LowRISC 参照**: lowRISC は組み合わせロジックの記述を `always_comb` に統一し、`always @*` は legacy コードとみなします。
- **良い例**:

```systemverilog
always_comb begin
  state_d = state_q;
  unique case (state_q)
    IDLE: if (req_i) state_d = BUSY;
    default: state_d = IDLE;
  endcase
end
```

- **悪い例**:

```systemverilog
always @* begin
  state_d = state_q;
end
```

- **追加のポイント**: `always_comb` は暗黙で LHS を初期化しないため、ブロック内で確実に初期化してください。`sv-mint` では `always @(*)` `always @ (*)` などの表記ゆれも検出対象です。

### `lang.no_always_latch`
- **検出条件**: `always_latch` キーワードを見つけたら即座に警告します。
- **代表メッセージ**: `` always_latch is discouraged; prefer flip-flops ``
- **主な対処**: 意図的なラッチであれば設計方針を見直し、必要ならルールを個別に無効化します。
- **LowRISC 参照**: lowRISC の Sequential Logic 節ではラッチ生成を禁止し、`always_ff` ベースの同期ロジックへ置き換えることを求めています。
- **良い例**: `always_ff @(posedge clk_i or negedge rst_ni)` を使い、状態保持をフロップで実装する。
- **悪い例**:

```systemverilog
always_latch begin
  if (enable) data_q <= data_d;
end
```

- **追加のポイント**: どうしてもアナログインターフェースでレベルセンシティブ動作が必要な場合は、ラッチである理由をドキュメント化し、`sv-mint.toml` 側で対象ファイルのみルール無効化する方法をとってください。

### `lang.always_ff_reset`
- **検出条件**: `always_ff` の感度リスト内に `negedge` を含むリセットが無い場合に報告します。
- **代表メッセージ**: `` always_ff should include asynchronous reset (negedge rst_n) ``
- **主な対処**: 組織のリセットスタイルに合わせて `negedge rst_n` などを必ず含めます。
- **LowRISC 参照**: lowRISC は `always_ff @(posedge clk_i or negedge rst_ni)` 形式を標準化しており、非同期アクティブローリセット `rst_ni` の存在を前提にしています。
- **良い例**:

```systemverilog
always_ff @(posedge clk_i or negedge rst_ni) begin
  if (!rst_ni) data_q <= '0;
  else data_q <= data_d;
end
```

- **悪い例**:

```systemverilog
always_ff @(posedge clk_i) begin
  data_q <= data_d;  // リセットが無い
end
```

- **追加のポイント**: 同期リセットを採用するプロジェクトでは `ruleset.override` でメッセージ文面を変更するか、`negedge` 以外の感度を許可する PR を検討してください。

### `lang.always_comb_at`
- **検出条件**: `always_comb` 宣言の後に `@` が続くケースを検知します。
- **代表メッセージ**: `` always_comb must not have sensitivity list ``
- **主な対処**: 感度リストを削除して純粋な `always_comb` としてください。
- **LowRISC 参照**: lowRISC では `always_comb` が自動感度リストを提供するため、`@(...)` を併記すると定義違反とみなされています。
- **良い例**:

```systemverilog
always_comb begin
  sum = a + b;
end
```

- **悪い例**:

```systemverilog
always_comb @(*) begin
  sum = a + b;
end
```

- **追加のポイント**: 自動修正時は `always @*` でなく `always_comb` に統一した上で感度リストを削除してください。`always_comb` の直後にコメントを置く場合は `// comb` のあと改行して `begin` を書くと視認性が上がります。
