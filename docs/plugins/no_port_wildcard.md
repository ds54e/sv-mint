# no_port_wildcard.py

- **Script**: `plugins/no_port_wildcard.py`
- **Stage**: `cst` (`mode = inline`)
- **Key Inputs**: `cst_ir.tokens`, `tok_kind_table`, `line_starts`
- **Rule**:
  - ``module.no_port_wildcard`` (warning): Catch `.*` port wildcards at the CST level

## Rule Details

### `module.no_port_wildcard`
- **Trigger**: Reports every `conn_wildcard` token with precise file/line/col info.
- **Message**: `` named port connections must not use .* wildcard ``
- **Remediation**: Expand connections to `.port(signal)` or update generators accordingly.
- **Notes**: Complements `module_inst_rules.py`; this variant survives preprocessing and macro expansion.
- **Good**:

```systemverilog
foo u_foo (
  .clk_i(clk_i),
  .rst_ni(rst_ni),
  .req_i(req_i)
);
```

- **Bad**:

```systemverilog
foo u_foo (.*);  // wildcard connection
```
