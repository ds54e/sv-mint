# format.begin_required.pp.py

- **Stage**: `pp_text`
- **Key Inputs**: Preprocessed `text`
- **Summary**: Require multiline control bodies to use `begin ... end`

## Details

### Trigger
Scans `if/for/foreach/while/repeat/forever` constructs. When their bodies span multiple lines but do not start with `begin`, the rule fires.
### Message
`` <keyword> body must start with begin when split across lines ``
### Remediation
Insert `begin` after the condition and add the matching `end`. For single statements, either keep them on one line or still wrap them for clarity.
### Notes
`else if` chains are analyzed with awareness of `else`, so match both sides. Because the rule uses preprocessed text, macros must expand to include the `begin` keyword.
### Good

```systemverilog
if (req_i) begin
  data_q <= data_d;
  ready_o <= 1'b1;
end
```

### Bad

```systemverilog
if (req_i)
  data_q <= data_d;
  ready_o <= 1'b1;  // multi-line body without begin/end
```
