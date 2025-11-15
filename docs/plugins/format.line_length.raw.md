# format.line_length.raw.py

- **Stage**: `raw_text`
- **Key Inputs**: LF-normalized `text`
- **Summary**: Flag lines longer than 100 columns

## Details

### Trigger
Measures each line and reports those exceeding `MAX_COLUMNS = 100`, pointing at column 101+.
### Message
`` line exceeds 100 columns (118) ``
### Remediation
Break long expressions, introduce temporaries, or wrap comments to stay within 100 columns.
### Notes
Threshold is fixed in code; tweak severity via the `severity` field in the corresponding `[[rule]]` entry if needed.
### Good

```systemverilog
assign addr_aligned = {addr_i[31:4], 4'b0};  // stays well under 100 columns
```

### Bad

```systemverilog
assign addr_aligned = {addr_i[31:4], 4'b0};  // this comment keeps going and going without wrapping so it easily exceeds the 100-column limit enforced by sv-mint
```
