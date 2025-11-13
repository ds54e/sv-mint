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

### `lang.prefer_always_comb`
- **検出条件**: `always @*` を検出して `always_comb` への置換を促します。
- **代表メッセージ**: `` use always_comb instead of always @* ``
- **主な対処**: 感度リストを書かずに `always_comb` を使用し、ツールの等価性を保ちます。

### `lang.no_always_latch`
- **検出条件**: `always_latch` キーワードを見つけたら即座に警告します。
- **代表メッセージ**: `` always_latch is discouraged; prefer flip-flops ``
- **主な対処**: 意図的なラッチであれば設計方針を見直し、必要ならルールを個別に無効化します。

### `lang.always_ff_reset`
- **検出条件**: `always_ff` の感度リスト内に `negedge` を含むリセットが無い場合に報告します。
- **代表メッセージ**: `` always_ff should include asynchronous reset (negedge rst_n) ``
- **主な対処**: 組織のリセットスタイルに合わせて `negedge rst_n` などを必ず含めます。

### `lang.always_comb_at`
- **検出条件**: `always_comb` 宣言の後に `@` が続くケースを検知します。
- **代表メッセージ**: `` always_comb must not have sensitivity list ``
- **主な対処**: 感度リストを削除して純粋な `always_comb` としてください。
