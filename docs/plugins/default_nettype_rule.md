# default_nettype_rule.py

- **Script**: `plugins/default_nettype_rule.py`
- **Stage**: `raw_text`
- **Key Inputs**: `text`
- **Rules**:
  | Rule ID | Severity | Summary |
  | --- | --- | --- |
  | `lang.default_nettype_missing` | warning | Require `` `default_nettype none`` in every file |
  | `lang.default_nettype_none` | warning | The first `default_nettype` must set the value to `none` |
  | `lang.default_nettype_placement` | warning | `default_nettype none` must appear near the file header |

## Rule Details

### `lang.default_nettype_missing`
Flags files that never declare `` `default_nettype``. The DVCodingStyle guidance (and most RTL style guides) expects `` `default_nettype none`` so that misspelled nets do not silently become implicit wires.

### `lang.default_nettype_none`
Even when a directive exists, it must explicitly set the value to `none`. Any other value (`wire`, `tri`, etc.) raises a violation so the toolchain doesn’t fall back to implicit nets mid-file.

### `lang.default_nettype_placement`
`default_nettype none` should appear close to the file header, before modules/packages/interfaces. This rule counts “significant” lines (ignoring blank lines and comments) and warns when the directive shows up after the first 20 such lines. This keeps the guard in place before any declarations are parsed.
