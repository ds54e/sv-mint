# global.local_define_undef

- **Stage**: `raw_text`
- **Key Inputs**: `text`
- **Shared Helpers**: `plugins/lib/global_define_ruleset.py`
- **Summary**: Require local macros to be undefined in the same file

## Details

### Trigger
Flags macros such as `_FOO` that reach EOF without a matching `` `undef``.
### Message
`` local macro <name> must be undefined after use ``
### Remediation
Insert `` `undef <name>`` in the same translation unit or move the macro to a tighter scope.
### Good

```systemverilog
`define _FOO(ARG) (ARG + 1)
assign data_o = `_FOO(data_i);
`undef _FOO
```

### Bad

```systemverilog
`define _FOO(ARG) (ARG + 1)
assign data_o = `_FOO(data_i);
// no `undef, so the macro leaks
```

### Additional Tips
Wrap the undef with guards such as `` `ifdef _FOO`` to avoid symbol clashes across includes.
