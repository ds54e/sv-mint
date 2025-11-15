# lang.default_nettype_placement

- **Stage**: `raw_text`
- **Key Inputs**: `text`
- **Shared Helpers**: `plugins/lib/default_nettype_ruleset.py`
- **Summary**: `default_nettype none` must appear near the file header

## Details

`default_nettype none` should appear close to the file header, before modules/packages/interfaces. This rule counts “significant” lines (ignoring blank lines and comments) and warns when the directive shows up after the first 20 such lines. This keeps the guard in place before any declarations are parsed.
### Good

```systemverilog
// SPDX header
`default_nettype none

module foo;
```

### Bad

```systemverilog
module foo;
  // ...
`default_nettype none  // too late
```
