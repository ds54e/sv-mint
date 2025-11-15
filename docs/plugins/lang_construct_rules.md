# lang_construct_rules.py

- **Script**: `plugins/lang_construct_rules.py`
- **Stage**: `raw_text`
- **Key Inputs**: `text`
- **Rules**:
  - ``lang.no_delays`` (warning): Ban `#5`-style delays in RTL
  - ``lang.prefer_always_comb`` (warning): Replace `always @*` with `always_comb`
  - ``lang.no_always_latch`` (warning): Discourage `always_latch`
  - ``lang.always_ff_reset`` (warning): Require asynchronous reset in `always_ff`
  - ``lang.always_comb_at`` (warning): Forbid sensitivity lists on `always_comb`

## Rule Details

### `lang.no_delays`
#### Trigger
Finds standalone `#` delay operators (excluding parameterized `#(...)` clauses).
#### Message
`` delay (#) constructs are not permitted ``
#### Remediation
Move timing behavior to testbenches or constraints; keep RTL delay-free.

### `lang.prefer_always_comb`
#### Trigger
Detects `always @*`/`always @ (*)` and suggests `always_comb`.
#### Message
`` use always_comb instead of always @* ``
#### Remediation
Convert to `always_comb` blocks with explicit default assignments.

### `lang.no_always_latch`
#### Trigger
Reports any `always_latch` keyword.
#### Message
`` always_latch is discouraged; prefer flip-flops ``
#### Remediation
Re-architect the logic with `always_ff` or justify the latch and disable the rule locally.

### `lang.always_ff_reset`
#### Trigger
Checks `always_ff` sensitivity lists for the literal substring `negedge`; if absent, the rule fires (posedge or renamed resets still warn).
#### Message
`` always_ff should include asynchronous reset (negedge rst_n) ``
#### Remediation
Add `or negedge rst_ni` (or update the plugin if a different reset style is required) so the sensitivity list contains `negedge`.

### `lang.always_comb_at`
#### Trigger
Flags `always_comb` followed by `@`.
#### Message
`` always_comb must not have sensitivity list ``
#### Remediation
Remove the explicit sensitivity list; `always_comb` already infers it.
#### Good

```systemverilog
always_comb begin
  state_d = state_q;
  unique case (opcode_i)
    ADD: state_d = ADD_EXEC;
    default: ;
  endcase
end

always_ff @(posedge clk_i or negedge rst_ni) begin
  if (!rst_ni) state_q <= IDLE;
  else state_q <= state_d;
end
```

#### Bad

```systemverilog
always @* state_d = next_state;  // prefer always_comb
always_latch begin
  state_q <= next_state;
end
always_ff @(posedge clk_i) begin  // missing negedge reset
  #1 state_q <= next_state;       // delay operator banned in RTL
end
always_comb @(posedge clk_i) data_d = data_q;  // sensitivity list on always_comb
```
