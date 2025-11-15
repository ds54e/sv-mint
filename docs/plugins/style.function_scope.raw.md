# style.function_scope.raw.py

- **Stage**: `raw_text`
- **Key Inputs**: `text`, `path`
- **Shared Helpers**: `plugins/lib/dv_text_ruleset.py`
- **Summary**: Functions inside packages/modules/interfaces must be `automatic` or `static`

## Details

### Trigger
Finds `function` declarations outside classes that omit both `automatic` and `static`.
### Message
`` function must declare automatic or static ``
### Remediation
Add the `automatic` keyword (or `static` when intentional) so lifetime semantics are explicit.
### Good

```systemverilog
function automatic int calc_checksum(input int data);
  return data ^ 32'hDEADBEEF;
endfunction
```

### Bad

```systemverilog
function int calc_checksum(input int data);  // missing automatic/static
  return data;
endfunction
```
