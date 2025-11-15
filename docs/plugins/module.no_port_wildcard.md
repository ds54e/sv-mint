# module.no_port_wildcard

- **Stage**: `cst` (`mode = inline`)
- **Key Inputs**: `cst_ir.tokens`, `tok_kind_table`, `line_starts`
- **Summary**: Catch `.*` port wildcards at the CST level

## Details

### Trigger
Reports every `conn_wildcard` token with precise file/line/col info.
### Message
`` named port connections must not use .* wildcard ``
### Remediation
Expand connections to `.port(signal)` or update generators accordingly.
### Notes
Complements `module_inst_rules.py`; this variant survives preprocessing and macro expansion.
### Good

```systemverilog
foo u_foo (
  .clk_i(clk_i),
  .rst_ni(rst_ni),
  .req_i(req_i)
);
```

### Bad

```systemverilog
foo u_foo (.*);  // wildcard connection
```
