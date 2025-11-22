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

### Good

```systemverilog
module m #(
  localparam int MyParam1 = 1,
  localparam int MyParam2 = 1, // reserved
  localparam int MyParam3 = 1 // will be used later
)(
  input logic [MyParam1:0] a
);
endmodule
```

```systemverilog
module stub;
  localparam int EnableDbg = 0;  // reserved
endmodule
```

### Bad

```
module m #(
  localparam int MyParam = 1
);
endmodule
```systemverilog
