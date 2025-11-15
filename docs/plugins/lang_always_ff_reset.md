# lang.always_ff_reset.raw.py

- **Stage**: `raw_text`
- **Key Inputs**: `text`
- **Shared Helpers**: `plugins/lib/lang_construct_ruleset.py`
- **Summary**: Require asynchronous reset in `always_ff`

## Details

### Trigger
Checks `always_ff` sensitivity lists for the literal substring `negedge`; if absent, the rule fires (posedge or renamed resets still warn).
### Message
`` always_ff should include asynchronous reset (negedge rst_n) ``
### Remediation
Add `or negedge rst_ni` (or update the plugin if a different reset style is required) so the sensitivity list contains `negedge`.
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
  state_q <= state_d;  // missing negedge reset
end
```
