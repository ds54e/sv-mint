# lang.case_requires_unique

- **Script**: `plugins/lang.case_requires_unique.cst.py`
- **Stage**: `cst` (`mode = inline`)
- **Key Inputs**: `cst_ir.tokens`, `tok_kind_table`, `line_starts`
- **Summary**: Recommend adding `unique` or `priority` to `case` statements

## Details

### Trigger
Parses each `CaseStatement` token stream; if `unique` or `priority` does not appear immediately before `case`, the rule fires.
### Message
`` case statements should use unique or priority ``
### Remediation
Use `unique case` for completeness or `priority case` when priority matters. Disable the rule if your spec intentionally omits modifiers.
### Notes
For constructs like `case inside`, only the first `case` is checked; add modifiers individually if needed.
### Good

```systemverilog
unique case (opcode_i)
  OP_ADD: res_d = a_i + b_i;
  OP_SUB: res_d = a_i - b_i;
  default: res_d = '0;
endcase
```

### Bad

```systemverilog
case (opcode_i)
  OP_ADD: res_d = a_i + b_i;
  OP_SUB: res_d = a_i - b_i;
  default: res_d = '0;
endcase  // missing unique/priority, coverage unclear
```

### Additional Tips
`priority case` still benefits from a `default` branch in the lowRISC flow. Consider `priority if` when it better communicates intent.
