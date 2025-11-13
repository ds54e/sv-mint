# decl_unused_param.py

- **Script**: `plugins/decl_unused_param.py`
- **Stage**: `ast`
- **Key Inputs**: `symbols` entries with `class == param`
- **Rule**:
  | Rule ID | Severity | Summary |
  | --- | --- | --- |
  | `decl.unused.param` | warning | Detect parameters whose reference count stays at zero |

## Rule Details

### `decl.unused.param`
- **Trigger**: Filters `symbols` for `class == param` and flags entries with `ref_count` (or `read_count`) equal to zero.
- **Message**: `` unused param <module>.<name> ``
- **Remediation**: Remove unused parameters or ensure top-level configuration knobs are actually referenced inside the module.
- **Notes**: Auto-generated code that allows dummy parameters can downgrade severity via `ruleset.override`.
- **LowRISC Reference**: Parameters should document module configurability; unused ones must be deleted.
- **Good**:

```systemverilog
module fifo #(parameter int Depth = 16) (
  input  logic [$clog2(Depth):0] addr_i,
  ...
);
```

- **Bad**:

```systemverilog
module fifo #(parameter int Depth = 16,
              parameter bit EnableDbg = 0) (
  input logic req_i
);
// EnableDbg is never referenced
```

- **Additional Tips**: `localparam` emitted by macros under conditionals often end up unused; keep the declaration inside the `ifdef` or tag it with attributes like `(* keep = "true" *)`. If the symbol still needs to exist, add a pattern such as `EnableDbg` to `ruleset.allowlist`.
