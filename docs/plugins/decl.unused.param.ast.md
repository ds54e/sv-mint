# decl.unused.param.ast.py

- **Stage**: `ast`
- **Key Inputs**: `symbols` entries with `class == param`
- **Summary**: Detect parameters whose reference count stays at zero

## Details

### Trigger
Filters `symbols` for `class == param` and flags entries with `ref_count` (or `read_count`) equal to zero.
### Message
`` unused param <module>.<name> ``
### Remediation
Remove unused parameters, ensure configuration knobs are referenced, or annotate intentional placeholders with inline comments containing `unused` (for example, `` parameter bit EnableDbg = 0  // unused ``).
### Notes
The rule treats both `parameter` and `localparam` symbols identically because the AST reports them under the `param` class.
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

### Additional Tips
Only comments on the declaration line are checked for the `unused` keyword. Macro-generated `localparam` entries should carry that inline note or be kept inside the guarding `ifdef`.
