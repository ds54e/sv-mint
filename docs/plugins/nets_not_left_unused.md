# nets_not_left_unused

- **Script**: `plugins/nets_not_left_unused.ast.py`
- **Stage**: `ast`
- **Key Inputs**: `symbols` entries with `class == net`
- **Summary**: Warn when declared nets are never read or written

## Details

### Message
`` unused net <module>.<name> ``
### Remediation
Delete unused nets or annotate intentional placeholders with inline comments containing `unused` (e.g., `` wire debug_tap; // unused ``).

### Limitations
- Implicit connections (`.*`, `.foo` shorthand) are not elaborated; they will be counted as unused.
- If the declaration line contains a comment with the words `used` or `reserved` (case-insensitive), the warning is suppressed.
### Good

```systemverilog
wire req_i;
wire ack_o;
wire busy;

assign busy = req_i & ack_o;  // net is driven and consumed
```

```systemverilog
wire debug_tap;  // unused  (explicit intent keeps lint quiet)
```

### Bad

```systemverilog
wire req_i;
wire ack_o;
wire debug_tap;  // declared but never read or written
```
