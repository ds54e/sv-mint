# comb_nb_in_alwayscomb.py

- **Script**: `plugins/comb_nb_in_alwayscomb.py`
- **Stage**: `cst`
- **Key Inputs**: `cst_ir.tokens`, `tok_kind_table`, `line_starts`, `pp_text`
- **Rule**:
  | Rule ID | Severity | Summary |
  | --- | --- | --- |
  | `comb.nb_in_alwayscomb` | warning | Ban non-blocking assignments (`<=`) inside `always_comb` |

## Rule Details

### `comb.nb_in_alwayscomb`
- **Trigger**: Identifies `always_comb` nodes and flags any `op_le` (`<=`) tokens within the block. Falls back to text scanning when token info is missing.
- **Message**: `` nonblocking '<=' inside always_comb ``
- **Remediation**: Use blocking `=` inside combinational logic; if state is required, move the logic to `always_ff`.
- **Notes**: When `sv-parser` updates token kinds, ensure `op_le` remains present in `tok_kind_table`.
- **LowRISC Reference**: The style guide explicitly forbids `<=` in `always_comb`, keeping these blocks purely combinational.
- **Good**:

```systemverilog
always_comb begin
  result_d = a_i ^ b_i;  // blocking assignments only
end
```

- **Bad**:

```systemverilog
always_comb begin
  result_q <= a_i ^ b_i;  // violates combinational semantics
end
```

- **Additional Tips**: Some tools infer flops when `<=` appears inside `always_comb`. Macros that hide the operator still trigger the rule after expansion, so provide separate helpers for blocking vs. non-blocking assignments.
