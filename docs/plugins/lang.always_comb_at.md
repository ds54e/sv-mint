# lang.always_comb_at

- **Stage**: `raw_text`
- **Key Inputs**: `text`
- **Shared Helpers**: `plugins/lib/lang_construct_ruleset.py`
- **Summary**: Forbid sensitivity lists on `always_comb`

## Details

### Trigger
Flags `always_comb` followed by `@`.
### Message
`` always_comb must not have sensitivity list ``
### Remediation
Remove the explicit sensitivity list; `always_comb` already infers it.
### Good

```systemverilog
always_comb begin
  data_d = data_q;
end
```

### Bad

```systemverilog
always_comb @(posedge clk_i) begin
  data_d = data_q;  // sensitivity list disallowed
end
```
