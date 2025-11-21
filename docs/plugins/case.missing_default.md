# case.missing_default

- **Script**: `plugins/case.missing_default.cst.py`
- **Stage**: `cst` (`mode = inline`)
- **Key Inputs**: `cst_ir.tokens`, `line_starts`, `pp_text`
- **Summary**: Warn when a `case` statement lacks a `default` label (except for `unique`/`unique0 case`)

## Details

### Trigger
Walks each CST `CaseStatement`; if no `default` token appears, the rule reports the first token location.
### Message
`` case statement must include a default item ``
### Remediation
Add a `default` branch when using plain `case`. `unique`/`unique0` cases are exempt (the rule does not check exhaustiveness).
### Notes
The pass inspects preprocessed `pp_text`, so macros that expand to `default` must emit the token after preprocessing.
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

### Additional Tips
When wrapping `default` in `begin/end`, keep the colon directly after the token. Align macro-generated `default` blocks with the surrounding case body to reduce review slips.
