# global.prefer_parameters

- **Script**: `plugins/global.prefer_parameters.raw.py`
- **Stage**: `raw_text`
- **Key Inputs**: `text`
- **Shared Helpers**: `plugins/lib/global_define_ruleset.py`
- **Summary**: Discourage non-underscored `` `define`` macros in favor of parameters

## Details

### Trigger
Reports any `` `define`` that does not start with `_`, discouraging project-wide macro switches.
### Message
`` use parameters instead of global macro `FOO``
### Remediation
Replace macros with module parameters or `localparam`. Lower the noise floor by setting `severity = "info"` in the corresponding `[[rule]]` entry when policy allows.
### Good

```systemverilog
module foo #(parameter bit EnableParity = 1'b1) (...);
```

### Bad

```systemverilog
`define ENABLE_PARITY 1
module foo (...);
  if (`ENABLE_PARITY) begin
    ...
  end
endmodule
```

### Additional Tips
When legacy IP needs macros, either refactor emitters to use `_FOO`-style locals or temporarily disable the rule via its `[[rule]]` entry during migration.
