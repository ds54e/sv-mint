# dpi_export_prefix.py

- **Script**: `plugins/dpi.export_prefix.raw.py`
- **Stage**: `raw_text`
- **Key Inputs**: `text`, `path`
- **Shared Helpers**: `plugins/lib/dv_text_ruleset.py`
- **Rule**:
  - ``dpi.export_prefix`` (warning): Exported DPI handles must start with `sv_dpi_`

## Rule Details

### `dpi.export_prefix`
#### Trigger
Checks DPI export declarations for the `sv_dpi_` prefix.
#### Message
`` exported DPI symbol must start with sv_dpi_ ``
#### Remediation
Rename exported tasks/functions accordingly.
#### Good

```systemverilog
export "DPI-C" task sv_dpi_alert;
```

#### Bad

```systemverilog
export "DPI-C" task alert_task;
```
