# lang_construct_rules.py

- **Script**: `plugins/lang_construct_rules.py`
- **Stage**: `raw_text`
- **Key Inputs**: `text`
- **Rules**:
  | Rule ID | Severity | Summary |
  | --- | --- | --- |
  | `lang.no_delays` | warning | Ban `#5`-style delays in RTL |
  | `lang.prefer_always_comb` | warning | Replace `always @*` with `always_comb` |
  | `lang.no_always_latch` | warning | Discourage `always_latch` |
  | `lang.always_ff_reset` | warning | Require asynchronous reset in `always_ff` |
  | `lang.always_comb_at` | warning | Forbid sensitivity lists on `always_comb` |

## Rule Details

### `lang.no_delays`
- **Trigger**: Finds standalone `#` delay operators (excluding parameterized `#(...)` clauses).
- **Message**: `` delay (#) constructs are not permitted ``
- **Remediation**: Move timing behavior to testbenches or constraints; keep RTL delay-free.

### `lang.prefer_always_comb`
- **Trigger**: Detects `always @*`/`always @ (*)` and suggests `always_comb`.
- **Message**: `` use always_comb instead of always @* ``
- **Remediation**: Convert to `always_comb` blocks with explicit default assignments.

### `lang.no_always_latch`
- **Trigger**: Reports any `always_latch` keyword.
- **Message**: `` always_latch is discouraged; prefer flip-flops ``
- **Remediation**: Re-architect the logic with `always_ff` or justify the latch and disable the rule locally.

### `lang.always_ff_reset`
- **Trigger**: Checks `always_ff` sensitivity lists for `negedge` resets; warns when missing.
- **Message**: `` always_ff should include asynchronous reset (negedge rst_n) ``
- **Remediation**: Add `or negedge rst_ni` (or your org’s reset style) to every sequential block.

### `lang.always_comb_at`
- **Trigger**: Flags `always_comb` followed by `@`.
- **Message**: `` always_comb must not have sensitivity list ``
- **Remediation**: Remove the explicit sensitivity list; `always_comb` already infers it.
