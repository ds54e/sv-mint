# Naming rules

- **Scripts**:
  - `plugins/naming.module_case.ast.py`
  - `plugins/naming.signal_case.ast.py`
  - `plugins/naming.port_case.ast.py`
  - `plugins/naming.port_suffix.ast.py`
  - `plugins/naming.no_numeric_suffix.ast.py`
  - `plugins/naming.suffix_order.ast.py`
  - `plugins/naming.clk_prefix.ast.py`
  - `plugins/naming.rst_active_low.ast.py`
  - `plugins/naming.clk_order.ast.py`
  - `plugins/naming.rst_before_clk.ast.py`
  - `plugins/naming.differential_pair.ast.py`
  - `plugins/naming.pipeline_sequence.ast.py`
  - `plugins/naming.parameter_upper.ast.py`
- **Stage**: `ast`
- **Key Inputs**: `decls`, `symbols`, `ports`
- **Shared Helpers**: `plugins/lib/naming_ruleset.py`
- **Rules**:
  - ``naming.module_case`` (warning): Modules must use lower_snake_case
  - ``naming.signal_case`` (warning): Signals/variables must use lower_snake_case
  - ``naming.port_case`` (warning): Ports follow lower_snake_case + direction suffix
  - ``naming.port_suffix`` (warning): `_i/_o/_io` suffixes must match port direction
  - ``naming.no_numeric_suffix`` (warning): Ban trailing `_42` numeric suffixes
  - ``naming.suffix_order`` (warning): Enforce `_ni/_no/_nio` suffix ordering
  - ``naming.clk_prefix`` (warning): Clock names must start with `clk`
  - ``naming.rst_active_low`` (warning): Reset names must end with `_n` (or `_ni/_no/_nio`)
  - ``naming.clk_order`` (warning): Ports must list clocks first
  - ``naming.rst_before_clk`` (warning): Resets must directly follow clocks
  - ``naming.differential_pair`` (warning): `_p` ports require matching `_n` ports
  - ``naming.pipeline_sequence`` (warning): `_q2`+ stages require preceding `_q<n-1>`
  - ``naming.parameter_upper`` (warning): Parameters must be UpperCamelCase

## Rule Details

### `naming.module_case`
#### Trigger
Flags `module` declarations whose identifiers are not lower_snake_case.
#### Message
`` module <name> must use lower_snake_case ``
#### Remediation
Rename modules so they start with a lowercase letter and only use lowercase letters, digits, or underscores.
#### Good

```systemverilog
module entropy_ctrl;
endmodule
```

#### Bad

```systemverilog
module EntropyCtrl;
endmodule
```

### `naming.signal_case`
#### Trigger
Checks nets and variables for lower_snake_case identifiers.
#### Message
`` signal <name> must use lower_snake_case ``
#### Remediation
Rename `logic`/`wire`/`reg` identifiers to lowercase snake case.
#### Good

```systemverilog
logic error_flag;
```

#### Bad

```systemverilog
logic errorFlag;
```

### `naming.port_case`
#### Trigger
Verifies that port names follow lower_snake_case before suffixes are considered.
#### Message
`` port <name> must use lower_snake_case ``
#### Remediation
Rename ports to lowercase snake case and then apply direction suffix rules.
#### Good

```systemverilog
input  logic req_i;
output logic gnt_o;
```

#### Bad

```systemverilog
input logic Req;
output logic Grant;
```

### `naming.port_suffix`
#### Trigger
Ensures `_i/_ni`, `_o/_no`, or `_io/_nio` suffixes match the declared port direction.
#### Message
`` port <name> must use suffix matching its direction ``
#### Remediation
Append `_i`, `_o`, or `_io` (with `_n` for active-low signals) so direction is obvious at call sites.
#### Good

```systemverilog
input  logic req_i;
input  logic rst_ni;
output logic data_o;
```

#### Bad

```systemverilog
input  logic req;
output logic data_out;
```

### `naming.no_numeric_suffix`
#### Trigger
Detects identifiers ending in `_<digits>`.
#### Message
`` <name> must not end with _<number> ``
#### Remediation
Use meaningful suffixes such as `_a/_b` or `_stage1/_stage2`, not raw numbers.
#### Good

```systemverilog
logic state_a, state_b;
```

#### Bad

```systemverilog
logic state_42;
```

### `naming.suffix_order`
#### Trigger
Catches split suffixes like `_n_i` or `_n_o`.
#### Message
`` combine reset and direction suffixes (e.g. rst_ni) ``
#### Remediation
Merge `_n` with `_i/_o/_io` to form `_ni/_no/_nio`.
#### Good

```systemverilog
logic rst_ni;
```

#### Bad

```systemverilog
logic rst_n_i;
```

### `naming.clk_prefix`
#### Trigger
Requires clock ports to start with `clk`.
#### Message
`` clock port <name> must start with 'clk' ``
#### Remediation
Rename to `clk_<domain>_<suffix>`.
#### Good

```systemverilog
input logic clk_core_i;
```

#### Bad

```systemverilog
input logic core_clk_i;
```

### `naming.rst_active_low`
#### Trigger
Ensures reset names end in `_n`, `_ni`, `_no`, or `_nio`.
#### Message
`` reset <name> must use active-low suffix `_n` ``
#### Remediation
Rename resets to `rst_ni`, `rst_no`, etc.
#### Good

```systemverilog
input logic rst_ni;
```

#### Bad

```systemverilog
input logic rst_i;
```

### `naming.clk_order`
#### Trigger
Checks that port lists declare all clocks before resets and data ports.
#### Message
`` clk ports must appear before other ports ``
#### Remediation
Group `clk*` ports at the top of the port list.
#### Good

```systemverilog
module dma_ctrl (
  input logic clk_core_i,
  input logic clk_bus_i,
  input logic rst_ni,
  input logic req_i
);
```

#### Bad

```systemverilog
module dma_ctrl (
  input logic rst_ni,
  input logic clk_core_i,
  input logic req_i
);
```

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

### `naming.differential_pair`
#### Trigger
Looks for `_p` ports without a matching `_n` sharing the same base name.
#### Message
`` differential pair missing companion <base>_n ``
#### Remediation
Declare both halves or rename the signal if it is not differential.
#### Good

```systemverilog
output logic tx_p_o;
output logic tx_n_o;
```

#### Bad

```systemverilog
output logic tx_p_o;
```

### `naming.pipeline_sequence`
#### Trigger
Ensures `_q2` and above have contiguous predecessor stages.
#### Message
`` pipeline stage <name> missing previous stage `` 
#### Remediation
Declare `_q`, `_q1`, `_q2`, etc., without skipping counts.
#### Good

```systemverilog
logic state_q;
logic state_q1;
logic state_q2;
```

#### Bad

```systemverilog
logic state_q;
logic state_q2;  // q1 missing
```

### `naming.parameter_upper`
#### Trigger
Flags `parameter` names that are not UpperCamelCase.
#### Message
`` parameter <name> must use UpperCamelCase ``
#### Remediation
Rename parameters to `DataWidth`, `NumAlerts`, etc.
#### Good

```systemverilog
parameter int DataWidth = 32;
```

#### Bad

```systemverilog
parameter int data_width = 32;
```
