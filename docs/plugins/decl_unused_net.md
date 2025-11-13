# decl_unused_net.py

- **Script**: `plugins/decl_unused_net.py`
- **Stage**: `ast`
- **Key Inputs**: `symbols` entries with `class == net`
- **Rule**:
  | Rule ID | Severity | Summary |
  | --- | --- | --- |
  | `decl.unused.net` | warning | Warn when declared nets are never read or written |

## Rule Details

### `decl.unused.net`
- **Trigger**: Selects `symbols` with `class="net"` where both `read_count` and `write_count` are zero, reporting the declaration location.
- **Message**: `` unused net <module>.<name> ``
- **Remediation**: Delete unused nets or rename future placeholders to something whitelisted such as `_unused`.
- **Notes**: AST data reflects the post-include source, so nets referenced only under conditional compilation may appear unused if `ignore_include` is enabled.
- **LowRISC Reference**: The style guide bans stray signals because they add confusion; this rule enforces that cleanup.
- **Good**:

```systemverilog
wire req_i;
wire ack_o;
wire busy;

assign busy = req_i & ack_o;  // net is driven and consumed
```

- **Bad**:

```systemverilog
wire req_i;
wire ack_o;
wire debug_tap;  // declared but never read or written
```

- **Additional Tips**: Comments like `/* unused */` do not suppress the warning. Either have generators emit names such as `unused_net_*` or disable the rule temporarily via its `[[rule]]` entry while migrating.
