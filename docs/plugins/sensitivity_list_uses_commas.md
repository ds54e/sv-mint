# sensitivity_list_uses_commas

- **Script**: `plugins/sensitivity_list_uses_commas.cst.py`
- **Stage**: `cst`
- **Key Inputs**: `cst_ir.tokens`, `line_starts`, `pp_text`
- **Summary**: Prefer comma-separated sensitivity lists instead of `or`

## Details

### Message
`` use ',' separators in sensitivity lists instead of 'or' ``
### Good

```systemverilog
`default_nettype none

module sensitivity_list_uses_commas_good;
  logic clk_i;
  logic rst_ni;
  logic data_d;
  logic data_q;

  always_ff @(posedge clk_i, negedge rst_ni) begin
    if (!rst_ni) data_q <= '0;
    else data_q <= data_d;
  end
endmodule

`default_nettype wire
```

### Bad

```systemverilog
`default_nettype none

module sensitivity_or_violation;
  logic clk_i;
  logic rst_ni;
  logic data_d;
  logic data_q;

  always_ff @(posedge clk_i or negedge rst_ni) begin
    if (!rst_ni) data_q <= '0;
    else data_q <= data_d;
  end
endmodule

`default_nettype wire
```
