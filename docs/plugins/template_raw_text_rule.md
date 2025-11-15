# template_raw_text_rule.py

- **Script**: `plugins/template.raw_text_marker.raw.py`
- **Stage**: `raw_text`
- **Key Inputs**: `text`
- **Summary**: Report placeholder markers left in templates

## Details

### Trigger
Searches for `__SV_MINT_TEMPLATE__` and reports the first occurrence as an informational violation.
### Message
`` template marker detected ``
### Remediation
Remove the marker after copying templates into production code.
### Notes
Only one location per file is reported. Split template files if you need multiple markers.
### Good

```systemverilog
// SPDX-License-Identifier: Apache-2.0
// DMA channel template instantiation (marker removed)
```

### Bad

```systemverilog
// __SV_MINT_TEMPLATE__ keep/remove?
```

### Additional Tips
Blocking template markers in CI prevents unfinished scaffolding from landing in commits. Update generators (e.g., `scripts/new_module.sh`) to strip or replace the marker automatically.
