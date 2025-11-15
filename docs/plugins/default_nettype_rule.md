# Default nettype rules

- **Scripts**:
  - `plugins/lang.default_nettype_missing.raw.py`
  - `plugins/lang.default_nettype_none.raw.py`
  - `plugins/lang.default_nettype_placement.raw.py`
  - `plugins/lang.default_nettype_reset.raw.py`
- **Stage**: `raw_text`
- **Key Inputs**: `text`
- **Shared Helpers**: `plugins/lib/default_nettype_ruleset.py`
- **Rules**:
  - ``lang.default_nettype_missing`` (warning): Require `` `default_nettype none`` in every file
  - ``lang.default_nettype_none`` (warning): The first `default_nettype` must set the value to `none`
  - ``lang.default_nettype_placement`` (warning): `default_nettype none` must appear near the file header
  - ``lang.default_nettype_reset`` (warning): Files must reset `default_nettype` back to `wire` near the end

## Rule Details

### `lang.default_nettype_missing`
Flags files that never declare `` `default_nettype``. The DVCodingStyle guidance (and most RTL style guides) expects `` `default_nettype none`` so that misspelled nets do not silently become implicit wires.
#### Good

```systemverilog
`default_nettype none

module foo;
endmodule

`default_nettype wire
```

#### Bad

```systemverilog
module foo;
endmodule  // no `default_nettype directive
```

### `lang.default_nettype_none`
Even when a directive exists, it must explicitly set the value to `none`. Any other value (`wire`, `tri`, etc.) raises a violation so the toolchain doesn’t fall back to implicit nets mid-file.
#### Good

```systemverilog
`default_nettype none
```

#### Bad

```systemverilog
`default_nettype wire  // must start at none
```

### `lang.default_nettype_placement`
`default_nettype none` should appear close to the file header, before modules/packages/interfaces. This rule counts “significant” lines (ignoring blank lines and comments) and warns when the directive shows up after the first 20 such lines. This keeps the guard in place before any declarations are parsed.
#### Good

```systemverilog
// SPDX header
`default_nettype none

module foo;
```

#### Bad

```systemverilog
module foo;
  // ...
`default_nettype none  // too late
```

### `lang.default_nettype_reset`
Because `default_nettype` stays in effect for all subsequent compilation units, each file must reset it back to `wire` at the end so other sources aren’t accidentally processed with `none`. This rule looks at the last `default_nettype` directive and warns when it doesn’t restore `wire`.
#### Good

```systemverilog
`default_nettype none
module foo; endmodule
`default_nettype wire
```

#### Bad

```systemverilog
`default_nettype none
module foo; endmodule
// missing reset to wire
```
