# naming_module_case.py

- **Script**: `plugins/naming.module_case.ast.py`
- **Stage**: `ast`
- **Key Inputs**: `decls`, `symbols`, `ports`
- **Shared Helpers**: `plugins/lib/naming_ruleset.py`
- **Rule**:
  - ``naming.module_case`` (warning): Modules must use lower_snake_case

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
