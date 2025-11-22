# case_has_default_branch

- **Script**: `plugins/case_has_default_branch.cst.py`
- **Stage**: `cst` (`mode = inline`)
- **Key Inputs**: `cst_ir.tokens`, `line_starts`, `pp_text`
- **Summary**: Warn when a `case` statement lacks a `default` label (except for `unique`/`unique0 case`)

## Details

### Message
`` case statement must include a default item ``
### Remediation
Add a `default` branch when using plain `case`. `unique`/`unique0` cases are exempt (the rule does not check exhaustiveness).
### Good

```systemverilog
unique case (state_q)  // unique → default not required
  IDLE:   state_d = START;
  START:  state_d = DONE;
endcase
```

```systemverilog
case (state_q)  // default required when not unique
  IDLE:   state_d = START;
  START:  state_d = DONE;
  default: state_d = IDLE;  // handle unexpected states
endcase
```

### Bad

```systemverilog
case (opcode_i)
  4'h0: alu_d = ADD;
  4'h1: alu_d = SUB;
endcase  // no default, unknown values pass silently
```
