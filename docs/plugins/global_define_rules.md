# global_define_rules.py

- **Script**: `plugins/global_define_rules.py`
- **Stage**: `raw_text`
- **Key Inputs**: `text`
- **Rules**:
  | Rule ID | Severity | Summary |
  | --- | --- | --- |
  | `global.local_define_undef` | warning | Require local macros to be undefined in the same file |
  | `global.prefer_parameters` | warning | Discourage non-underscored `` `define`` macros in favor of parameters |

## Rule Details

### `global.local_define_undef`
- **Trigger**: Flags macros such as `_FOO` that reach EOF without a matching `` `undef``.
- **Message**: `` local macro <name> must be undefined after use ``
- **Remediation**: Insert `` `undef <name>`` in the same translation unit or move the macro to a tighter scope.
- **LowRISC Reference**: Local macros carry an underscore prefix and must be undefined before the file ends.
- **Good**:

```systemverilog
`define _FOO(ARG) (ARG + 1)
assign data_o = `_FOO(data_i);
`undef _FOO
```

- **Bad**:

```systemverilog
`define _FOO(ARG) (ARG + 1)
assign data_o = `_FOO(data_i);
// no `undef, so the macro leaks
```

- **Additional Tips**: Wrap the undef with guards such as `` `ifdef _FOO`` to avoid symbol clashes across includes.

### `global.prefer_parameters`
- **Trigger**: Reports any `` `define`` that does not start with `_`, discouraging project-wide macro switches.
- **Message**: `` use parameters instead of global macro `FOO``
- **Remediation**: Replace macros with module parameters or `localparam`. Adjust severity via `ruleset.override` if policy allows.
- **LowRISC Reference**: The guide favors parameters for configurability and limits global macros to a tiny curated set.
- **Good**:

```systemverilog
module foo #(parameter bit EnableParity = 1'b1) (...);
```

- **Bad**:

```systemverilog
`define ENABLE_PARITY 1
module foo (...);
  if (`ENABLE_PARITY) begin
    ...
  end
endmodule
```

- **Additional Tips**: When legacy IP needs macros, permit only certain prefixes via `ruleset.allowlist` (e.g., `^OPENTITAN_`).
