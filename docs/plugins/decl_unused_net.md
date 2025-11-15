# decl_unused_net.py

- **Script**: `plugins/decl_unused_net.py`
- **Stage**: `ast`
- **Key Inputs**: `symbols` entries with `class == net`
- **Rule**:
  - ``decl.unused.net`` (warning): Warn when declared nets are never read or written

## Rule Details

### `decl.unused.net`
#### Trigger
Selects `symbols` with `class="net"` where both `read_count` and `write_count` are zero, reporting the declaration location.
#### Message
`` unused net <module>.<name> ``
#### Remediation
Delete unused nets or annotate intentional placeholders with inline comments containing `unused` (e.g., `` wire debug_tap; // unused ``).
#### Notes
AST data reflects the post-include source, so nets referenced only under conditional compilation may appear unused if `ignore_include` is enabled.
#### Good

```systemverilog
wire req_i;
wire ack_o;
wire busy;

assign busy = req_i & ack_o;  // net is driven and consumed
```

```systemverilog
wire debug_tap;  // unused  (explicit intent keeps lint quiet)
```

#### Bad

```systemverilog
wire req_i;
wire ack_o;
wire debug_tap;  // declared but never read or written
```

#### Additional Tips
Only comments on the declaration line are considered; multi-line blocks or preceding comments do not suppress the warning. When mass-migrating generated RTL, disable the rule via its `[[rule]]` entry so real leftovers remain visible.
