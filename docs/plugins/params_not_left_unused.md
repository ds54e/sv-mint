# params_not_left_unused

- **Script**: `plugins/params_not_left_unused.ast.py`
- **Stage**: `ast`
- **Key Inputs**: `symbols` entries with `class == param`
- **Summary**: Detect parameters whose reference count stays at zero

## Details

### Message
`` unused param <module>.<name> ``
### Good

```systemverilog
module m #(
  parameter int MyParam1 = 1,
  parameter int MyParam2 = 1, // reserved
  parameter int MyParam3 = 1 // will be used later
)(
  input logic [MyParam1:0] a
);
endmodule
```

### Bad

```systemverilog
module m #(
  parameter int MyParam = 1
);
endmodule
```
