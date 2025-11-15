# width_literal_rules.py

- **Script**: `plugins/width_literal_rules.py`
- **Stage**: `raw_text`
- **Key Inputs**: `text`
- **Rule**:
  - ``width.unsized_base_literal`` (warning): Ban base literals without explicit widths

## Rule Details

### `width.unsized_base_literal`
#### Trigger
Regex `(?<![0-9_])'(b|B|d|D|h|H|o|O)` finds `'hFF`-style literals lacking a width.
#### Message
`` base literal must include explicit width (e.g. 8'hFF) ``
#### Remediation
Add widths (`8'h`, `4'd`, etc.) to every base literal.
#### Additional Tips
Use underscores for readability (`32'hDEAD_BEEF`) and move constants into `localparam` for reuse.
#### Good

```systemverilog
assign mask_o = 8'hFF;
localparam logic [31:0] MagicValue = 32'hDEAD_BEEF;
```

#### Bad

```systemverilog
assign mask_o = 'hFF;      // missing width
localparam MagicValue = 'd10;  // unsized decimal literal
```
