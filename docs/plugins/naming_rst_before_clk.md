# naming_rst_before_clk.py

- **Script**: `plugins/naming.rst_before_clk.ast.py`
- **Stage**: `ast`
- **Key Inputs**: `decls`, `symbols`, `ports`
- **Shared Helpers**: `plugins/lib/naming_ruleset.py`
- **Rule**:
  - ``naming.rst_before_clk`` (warning): Resets must directly follow clocks

## Rule Details

### `naming.rst_before_clk`
#### Trigger
Warns when resets are listed before any clock ports or when other ports intervene between the clock and reset groups.
#### Message
`` rst ports must follow clock ports without other signals in between ``
#### Remediation
Place all resets immediately after the final clock entry.
#### Good

```systemverilog
module dma_ctrl (
  input logic clk_core_i,
  input logic clk_bus_i,
  input logic rst_ni,
  input logic rst_async_ni,
  input logic req_i
);
```

#### Bad

```systemverilog
module dma_ctrl (
  input logic clk_core_i,
  input logic req_i,
  input logic rst_ni
);
```
