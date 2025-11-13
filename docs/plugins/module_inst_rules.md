# module_inst_rules.py

- **Script**: `plugins/module_inst_rules.py`
- **Stage**: `raw_text`
- **Key Inputs**: `text`
- **Rules**:
  | Rule ID | Severity | Summary |
  | --- | --- | --- |
  | `module.no_port_wildcard` | warning | Ban `.*` wildcard port connections |
  | `module.named_ports_required` | warning | Require named `.port(signal)` connections |

## Rule Details

### `module.no_port_wildcard`
- **Trigger**: Regex `\.*` catches wildcard port hookups.
- **Message**: `` avoid .* wildcard in module instantiations ``
- **Remediation**: Expand to explicit named ports to prevent silent autowiring.
- **LowRISC Reference**: Wildcards are prohibited; list every port.

### `module.named_ports_required`
- **Trigger**: Detects instantiations that begin with positional arguments (no `.` inside the port list).
- **Message**: `` use named port connections instead of positional arguments ``
- **Remediation**: Rewrite as `.clk(clk)` style to remove ordering hazards.
- **Notes**: Formatting tools such as `verible-verilog-format --named-port-formatting` help during migrations.
