# header_comment_rule.py

- **Script**: `plugins/header_comment_rule.py`
- **Stage**: `raw_text`
- **Key Inputs**: `text`
- **Rules**:
  - ``header.missing_spdx`` (warning): Require an SPDX identifier near the top of the file
  - ``header.missing_comment`` (warning): Require a header comment within the first five lines

## Rule Details

### `header.missing_spdx`
#### Trigger
Scans the first 200 characters for `SPDX-License-Identifier`; reports the file start when absent.
#### Message
`` file should include SPDX-License-Identifier header ``
#### Remediation
Add lines such as `// SPDX-License-Identifier: Apache-2.0` near the top.
#### Good

```systemverilog
// SPDX-License-Identifier: Apache-2.0
// DMA channel control logic
```

#### Bad

```systemverilog
// DMA channel control logic  // missing SPDX
```

#### Additional Tips
Embed the SPDX line in generator templates so emitted files stay compliant.

### `header.missing_comment`
#### Trigger
If the first five lines contain no `//` comment, the rule fires.
#### Message
`` file header should include descriptive comment ``
#### Remediation
Summarize module purpose, contacts, or key context at the top of the file.
#### Good

```systemverilog
// Control logic for entropy distribution network.
module entropy_ctrl (...);
```

#### Bad

```systemverilog
module entropy_ctrl (...);
```

#### Additional Tips
Mention module names, roles, and dependency URLs when useful. Generators should emit these comments automatically.
