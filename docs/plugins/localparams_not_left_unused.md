# localparams_not_left_unused

- **Script**: `plugins/localparams_not_left_unused.ast.py`
- **Stage**: `ast`
- **Key Inputs**: `symbols` entries with `class == localparam`
- **Summary**: Detect localparams whose reference count stays at zero

## Details

### Message
`` unused localparam <module>.<name> ``
### Remediation
Remove unused localparams, ensure configuration knobs are referenced, or annotate intentional placeholders with inline comments containing `used` or `reserved`.

### Limitations
- A declaration-line comment containing the words `used` or `reserved` (case-insensitive) suppresses this warning.

### Good

```systemverilog
module foo;
  localparam int Depth = 16;
  logic [Depth-1:0] data;
  assign data = {Depth{1'b0}};
endmodule
```

```systemverilog
module stub;
  localparam int EnableDbg = 0;  // reserved
endmodule
```

### Bad

```systemverilog
module fifo;
  localparam bit EnableDbg = 0;
  // EnableDbg is never referenced
endmodule
```
