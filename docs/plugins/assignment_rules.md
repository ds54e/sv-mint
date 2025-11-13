# assignment_rules.py

- **Script**: `plugins/assignment_rules.py`
- **Stage**: `ast`
- **Key Inputs**: `assigns` (each entry includes `module`, `lhs`, `op`, and locations)
- **Rules**:
  | Rule ID | Severity | Summary |
  | --- | --- | --- |
  | `flow.multiple_nonblocking` | warning | Report multiple non-blocking assignments to the same LHS within a module |

## Rule Details

### `flow.multiple_nonblocking`
- **Trigger**: Groups AST `assigns` with `op == nonblocking` by `(module, lhs)` and reports the second and later assignments.
- **Message**: `` multiple nonblocking assignments to <lhs> ``
- **Remediation**: Ensure only one `<=` drives a flop per clock domain. If you intentionally assign the same flop in multiple blocks, refactor so one side writes `state_d` (or uses `=`) and keep a single `<=`.
- **Notes**: Unless disabled in `sv-mint.toml`, the rule aggregates across hierarchical generates as long as the module/LHS pair matches, so double-check emitted code.
- **LowRISC Reference**: The Sequential Logic Process section requires a single `<=` per flop; this rule enforces that expectation.
- **Good**:

```systemverilog
always_ff @(posedge clk_i or negedge rst_ni) begin
  if (!rst_ni) begin
    state_q <= IDLE;
  end else begin
    state_q <= state_d;  // one assignment site
  end
end
```

- **Bad**:

```systemverilog
always_ff @(posedge clk_i) begin
  state_q <= state_d;
  if (flush_i) state_q <= IDLE;  // second <= in the same clock
end
```

- **Additional Tips**: Tracking also happens inside `generate` blocks, so repeated `genvar` instances still conflict. Prefer computing `state_d` via `unique case` and issuing a single `<=` at the end to avoid false positives.
