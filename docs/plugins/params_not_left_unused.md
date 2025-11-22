# params_not_left_unused

- **Script**: `plugins/params_not_left_unused.ast.py`
- **Stage**: `ast`
- **Key Inputs**: `symbols` entries with `class == param`
- **Summary**: Detect parameters whose reference count stays at zero

## Details

### Message
`` unused param <module>.<name> ``
### Remediation
Remove unused parameters, ensure configuration knobs are referenced, or annotate intentional placeholders with inline comments containing `used` or `reserved`. Localparams are covered by `localparams_not_left_unused`.

### Limitations
- A declaration-line comment containing the words `used` or `reserved` (case-insensitive) suppresses this warning.
### Good

```systemverilog
module fifo #(parameter int Depth = 16) (
  input  logic [$clog2(Depth):0] addr_i,
  ...
);
```

```systemverilog
module stub #(
  parameter bit EnableDbg = 0  // unused (intentionally reserved knob)
) ();
endmodule
```

### Bad

```systemverilog
module fifo #(parameter int Depth = 16,
              parameter bit EnableDbg = 0) (
  input  logic [$clog2(Depth)-1:0] addr_i
);

logic [Depth-1:0] mem_q;
// ... implementation never looks at EnableDbg
```
