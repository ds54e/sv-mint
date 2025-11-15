# lang_no_delays.py

- **Script**: `plugins/lang.no_delays.raw.py`
- **Stage**: `raw_text`
- **Key Inputs**: `text`
- **Shared Helpers**: `plugins/lib/lang_construct_ruleset.py`
- **Summary**: Ban `#5`-style delays in RTL

## Details

### Trigger
Finds standalone `#` delay operators (excluding parameterized `#(...)` clauses).
### Message
`` delay (#) constructs are not permitted ``
### Remediation
Move timing behavior to testbenches or constraints; keep RTL delay-free.
### Good

```systemverilog
always_ff @(posedge clk_i or negedge rst_ni) begin
  if (!rst_ni) state_q <= IDLE;
  else state_q <= state_d;
end
```

### Bad

```systemverilog
always_ff @(posedge clk_i) begin
  #1 state_q <= state_d;  // delay not allowed in RTL
end
```
