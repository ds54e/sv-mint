# dpi_import_prefix.py

- **Script**: `plugins/dpi.import_prefix.raw.py`
- **Stage**: `raw_text`
- **Key Inputs**: `text`, `path`
- **Shared Helpers**: `plugins/lib/dv_text_ruleset.py`
- **Summary**: Imported DPI symbols must start with `c_dpi_`

## Details

### Trigger
Inspects DPI import statements and verifies identifiers start with `c_dpi_`.
### Message
`` imported DPI symbol must start with c_dpi_ ``
### Remediation
Rename imported C functions with the `c_dpi_` prefix.
### Good

```systemverilog
import "DPI-C" function int c_dpi_hash(input int data);
```

### Bad

```systemverilog
import "DPI-C" function int hash(input int data);
```
