# Language construct rules

- **Scripts**: `plugins/lang.*.raw.py`
- **Stage**: `raw_text`
- **Key Inputs**: `text`
- **Shared Helpers**: `plugins/lib/lang_construct_ruleset.py`
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
#### Good

```systemverilog
always_ff @(posedge clk_i or negedge rst_ni) begin
  if (!rst_ni) state_q <= IDLE;
  else state_q <= state_d;
end
```

#### Bad

```systemverilog
always_ff @(posedge clk_i) begin
  #1 state_q <= state_d;  // delay not allowed in RTL
end
```

### `lang.prefer_always_comb`
#### Trigger
Detects `always @*`/`always @ (*)` and suggests `always_comb`.
#### Message
`` use always_comb instead of always @* ``
#### Remediation
Convert to `always_comb` blocks with explicit default assignments.
#### Good

```systemverilog
always_comb begin
  state_d = state_q;
  unique case (opcode_i)
    ADD: state_d = ADD_EXEC;
    default: ;
  endcase
end
```

#### Bad

```systemverilog
always @* begin
  state_d = next_state;  // missing always_comb
end
```

### `lang.no_always_latch`
#### Trigger
Reports any `always_latch` keyword.
#### Message
`` always_latch is discouraged; prefer flip-flops ``
#### Remediation
Re-architect the logic with `always_ff` or justify the latch and disable the rule locally.
#### Good

```systemverilog
always_ff @(posedge clk_i or negedge rst_ni) begin
  if (!rst_ni) state_q <= IDLE;
  else state_q <= state_d;
end
```

#### Bad

```systemverilog
always_latch begin
  state_q <= state_d;  // latch discouraged
end
```

### `lang.always_ff_reset`
#### Trigger
Checks `always_ff` sensitivity lists for the literal substring `negedge`; if absent, the rule fires (posedge or renamed resets still warn).
#### Message
`` always_ff should include asynchronous reset (negedge rst_n) ``
#### Remediation
Add `or negedge rst_ni` (or update the plugin if a different reset style is required) so the sensitivity list contains `negedge`.
#### Good

```systemverilog
always_ff @(posedge clk_i or negedge rst_ni) begin
  if (!rst_ni) state_q <= IDLE;
  else state_q <= state_d;
end
```

#### Bad

```systemverilog
always_ff @(posedge clk_i) begin
  state_q <= state_d;  // missing negedge reset
end
```

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
  data_d = data_q;
end
```

#### Bad

```systemverilog
always_comb @(posedge clk_i) begin
  data_d = data_q;  // sensitivity list disallowed
end
```
