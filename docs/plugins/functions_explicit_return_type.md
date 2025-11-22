# functions_explicit_return_type

- **Stage**: `cst`
- **Script**: `plugins/functions_explicit_return_type.cst.py`
- **What it checks**: Flags `function` declarations that rely on implicit return types (e.g., `function foo(...);`) instead of an explicit type like `function logic foo(...);`.
- **Rationale**: Implicit return types can hide signedness and width, leading to mismatches between callers and implementations.
- **How to fix**: Add an explicit return type to the function header, including signedness and width as needed (`function logic signed [3:0] foo(...);`).
