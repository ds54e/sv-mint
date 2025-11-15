# lang_no_always_latch.py

- **Script**: `plugins/lang.no_always_latch.raw.py`
- **Stage**: `raw_text`
- **Key Inputs**: `text`
- **Shared Helpers**: `plugins/lib/lang_construct_ruleset.py`
- **Rule**:
  - ``lang.no_always_latch`` (warning): Discourage `always_latch`

## Rule Details

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
