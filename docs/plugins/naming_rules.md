# naming_rules.py

- **Script**: `plugins/naming_rules.py`
- **Stage**: `ast`
- **Key Inputs**: `decls`, `symbols`, `ports`
- **Rules**:
  | Rule ID | Severity | Summary |
  | --- | --- | --- |
  | `naming.module_case` | warning | Modules must use lower_snake_case |
  | `naming.signal_case` | warning | Signals/variables must use lower_snake_case |
  | `naming.port_case` | warning | Ports follow lower_snake_case + direction suffix |
  | `naming.no_numeric_suffix` | warning | Ban trailing `_42` numeric suffixes |
  | `naming.suffix_order` | warning | Enforce `_ni/_no/_nio` suffix ordering |
  | `naming.clk_prefix` | warning | Clock names must start with `clk` |
  | `naming.rst_active_low` | warning | Reset names must end with `_n` (or `_ni/_no/_nio`) |
  | `naming.clk_order` | warning | Ports must list clocks first |
  | `naming.rst_before_clk` | warning | Resets must directly follow clocks |
  | `naming.differential_pair` | warning | `_p` ports require matching `_n` ports |
  | `naming.pipeline_sequence` | warning | `_q2`+ stages require preceding `_q<n-1>` |
  | `naming.parameter_upper` | warning | Parameters must be UpperCamelCase |

## Rule Highlights

### Lower-Snake Case (`naming.module_case`, `naming.signal_case`, `naming.port_case`)
- **Trigger**: Reports declarations not matching `^[a-z0-9_]+$` with direction suffixes as needed.
- **Fix**: Rename modules, signals, and ports to `lower_snake_case` (e.g., `dma_ctrl`, `req_i`).

### Numeric and Suffix Rules (`naming.no_numeric_suffix`, `naming.suffix_order`)
- **Trigger**: Detects names ending in `_\d+` or split suffixes like `_n_i`.
- **Fix**: Use meaningful suffixes (`_a/_b`, `_q1/_q2`) and merge `_n` with direction (`rst_ni`).

### Clock and Reset Rules (`naming.clk_prefix`, `naming.rst_active_low`, `naming.clk_order`, `naming.rst_before_clk`)
- **Trigger**: Ensures clock names begin with `clk`, resets end with `_n` plus direction, and port lists order `clk` before `rst` before everything else.
- **Fix**: Rename to `clk_core_i`, `rst_ni`, and reorder the port declarations accordingly.

### Differential Pairs (`naming.differential_pair`)
- **Trigger**: Finds `_p` ports lacking a matching `_n` with the same base name.
- **Fix**: Declare both halves (`tx_p_o` and `tx_n_o`) or drop the `_p` naming if not differential.

### Pipeline Stages (`naming.pipeline_sequence`)
- **Trigger**: Requires `_q2` and beyond to have contiguous predecessors.
- **Fix**: Declare `data_q`, `data_q1`, `data_q2`, etc., without skipping numbers.

### Parameters (`naming.parameter_upper`)
- **Trigger**: Flags `parameter` names that are not UpperCamelCase.
- **Fix**: Rename to `DataWidth`, `NumAlerts`, etc., leaving ALLCAPS for `localparam` only.
