# assignment_rules.py

- **Script**: `plugins/flow.multiple_nonblocking.ast.py`
- **Stage**: `ast`
- **Key Inputs**: `assigns` (each entry includes `module`, `lhs`, `op`, and locations)
- **Rules**:
  - ``flow.multiple_nonblocking`` (warning): Report multiple non-blocking assignments to the same LHS within a module

## Rule Details

### `flow.multiple_nonblocking`
#### Trigger
Groups AST `assigns` with `op == nonblocking` by `(module, lhs)` and reports the second and later assignments.
#### Message
`` multiple nonblocking assignments to <lhs> ``
#### Remediation
Ensure only one `<=` drives a flop per clock domain. If you intentionally assign the same flop in multiple blocks, refactor so one side writes `state_d` (or uses `=`) and keep a single `<=`.
#### Notes
The rule inspects the AST, so it catches repeated `<=` even when they live in plain `always @(posedge clk)` blocks or macro-expanded logic. Unless disabled in `sv-mint.toml`, it also aggregates across hierarchical generates as long as the module/LHS pair matches, so double-check emitted code.
#### Good

```systemverilog
always_comb begin
  state_d = state_q;
  if (flush_i) begin
    state_d = IDLE;  // compute next state with blocking assignments
  end
end

always_ff @(posedge clk_i or negedge rst_ni) begin
  if (!rst_ni) state_q <= IDLE;
  else state_q <= state_d;  // single nonblocking assignment site
end
```

#### Bad

```systemverilog
always_ff @(posedge clk_i) begin
  if (flush_i) begin
    state_q <= IDLE;  // first <= to state_q
  end
end

always_ff @(posedge clk_i) begin
  state_q <= state_d;  // second <= in the same module
end
```

#### Additional Tips
Tracking also happens inside `generate` blocks, so repeated `genvar` instances still conflict. Prefer computing `state_d` via `unique case` and issuing a single `<=` at the end to avoid false positives.
