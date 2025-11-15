# style_function_scope.py

- **Script**: `plugins/style.function_scope.raw.py`
- **Stage**: `raw_text`
- **Key Inputs**: `text`, `path`
- **Shared Helpers**: `plugins/lib/dv_text_ruleset.py`
- **Rule**:
  - ``style.function_scope`` (warning): Functions inside packages/modules/interfaces must be `automatic` or `static`

## Rule Details

### `style.function_scope`
#### Trigger
Finds `function` declarations outside classes that omit both `automatic` and `static`.
#### Message
`` function must declare automatic or static ``
#### Remediation
Add the `automatic` keyword (or `static` when intentional) so lifetime semantics are explicit.
#### Good

```systemverilog
function automatic int calc_checksum(input int data);
  return data ^ 32'hDEADBEEF;
endfunction
```

#### Bad

```systemverilog
function int calc_checksum(input int data);  // missing automatic/static
  return data;
endfunction
```
