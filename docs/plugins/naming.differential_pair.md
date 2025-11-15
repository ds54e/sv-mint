# naming.differential_pair

- **Stage**: `ast`
- **Key Inputs**: `decls`, `symbols`, `ports`
- **Shared Helpers**: `plugins/lib/naming_ruleset.py`
- **Summary**: `_p` ports require matching `_n` ports

## Details

### Trigger
Looks for `_p` ports without a matching `_n` sharing the same base name.
### Message
`` differential pair missing companion <base>_n ``
### Remediation
Declare both halves or rename the signal if it is not differential.
### Good

```systemverilog
output logic tx_p_o;
output logic tx_n_o;
```

### Bad

```systemverilog
output logic tx_p_o;
```
