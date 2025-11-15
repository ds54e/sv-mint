# header.missing_spdx.raw.py

- **Stage**: `raw_text`
- **Key Inputs**: `text`
- **Shared Helpers**: `plugins/lib/header_comment_ruleset.py`
- **Summary**: Require an SPDX identifier near the top of the file

## Details

### Trigger
Scans the first 200 characters for `SPDX-License-Identifier`; reports the file start when absent.
### Message
`` file should include SPDX-License-Identifier header ``
### Remediation
Add lines such as `// SPDX-License-Identifier: Apache-2.0` near the top.
### Good

```systemverilog
// SPDX-License-Identifier: Apache-2.0
// DMA channel control logic
```

### Bad

```systemverilog
// DMA channel control logic  // missing SPDX
```

### Additional Tips
Embed the SPDX line in generator templates so emitted files stay compliant.
