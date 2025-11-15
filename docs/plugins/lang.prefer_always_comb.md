# lang.prefer_always_comb

- **Script**: `plugins/lang.prefer_always_comb.py`
- **Stage**: `raw_text`
- **Key Inputs**: `text`
- **Shared Helpers**: `plugins/lib/lang_construct_ruleset.py`
- **Summary**: Replace `always @*` with `always_comb`

## Details

### Trigger
Detects `always @*`/`always @ (*)` and suggests `always_comb`.
### Message
`` use always_comb instead of always @* ``
### Remediation
Convert to `always_comb` blocks with explicit default assignments.
### Good

```systemverilog
always_comb begin
  state_d = state_q;
  unique case (opcode_i)
    ADD: state_d = ADD_EXEC;
    default: ;
  endcase
end
```

### Bad

```systemverilog
always @* begin
  state_d = next_state;  // missing always_comb
end
```
