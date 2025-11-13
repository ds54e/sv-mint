# debug_ping.py

- **Script**: `plugins/debug_ping.py`
- **Stage**: Any (reports whichever `stage` the host passes)
- **Key Inputs**: `payload.ast`, `payload.symbols`, and related stats
- **Rule**:
  | Rule ID | Severity | Summary |
  | --- | --- | --- |
  | `debug.ping` | warning | Echoes the stage name and symbol count to confirm end-to-end wiring |

## Rule Details

### `debug.ping`
- **Trigger**: Always emits one violation showing the current stage and the number of `symbols` (or `ast.symbols`) available.
- **Message**: `` debug ping: stage=ast, symbols=42 ``
- **Remediation**: Remove this script from production `sv-mint.toml` configurations.
- **Notes**: Useful when extending payloads to ensure the data arrives as expected. Override severity via `to_viol` if necessary.
- **LowRISC Reference**: Not directly tied to the guide; treat it like any debug artifact and exclude it from release flows.
- **Good** (development only):

```toml
[ruleset.scripts]
debug_ping = { path = "plugins/debug_ping.py", stage = "ast", enabled = true }

[profile.release.ruleset.scripts]
debug_ping = { enabled = false }
```

- **Bad** (left enabled in production):

```toml
[ruleset.scripts]
debug_ping = { path = "plugins/debug_ping.py", stage = "ast" }
# No release override, so warnings appear permanently
```

- **Additional Tips**: If CI shows `debug.ping`, important diagnostics may be buried. Disable the rule in PR pipelines. Feel free to include other payload stats in the message while experimenting.
