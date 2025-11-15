# naming.clk_order.ast.py

- **Stage**: `ast`
- **Key Inputs**: `decls`, `symbols`, `ports`
- **Shared Helpers**: `plugins/lib/naming_ruleset.py`
- **Summary**: Ports must list clocks first

## Details

### Trigger
Checks that port lists declare all clocks before resets and data ports.
### Message
`` clk ports must appear before other ports ``
### Remediation
Group `clk*` ports at the top of the port list.
### Good

```systemverilog
module dma_ctrl (
  input logic clk_core_i,
  input logic clk_bus_i,
  input logic rst_ni,
  input logic req_i
);
```

### Bad

```systemverilog
module dma_ctrl (
  input logic rst_ni,
  input logic clk_core_i,
  input logic req_i
);
```
