# functions_explicit_arg_types

- **Stage**: `cst`
- **Script**: `plugins/functions_explicit_arg_types.cst.py`
- **What it checks**: Flags function arguments that omit explicit data types (e.g., `input a`), or use an implicit type equal to the identifier token.
- **Rationale**: Untyped arguments default to 1-bit logic and can silently mis-size or mis-sign values when the function is reused.
- **How to fix**: Annotate each argument with a full data type, including width and signedness as appropriate (`input logic signed [7:0] data_i`).
