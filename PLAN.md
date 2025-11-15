# Rule File Refactor Plan

## Overview
We will migrate bundled rule scripts to a "one rule per file" layout and derive both script path and stage from the filename. Each rule file will follow the pattern `<rule_id>.<stage>.py`, where `<stage>` is one of `raw`, `pp`, `cst`, or `ast`. When a `[[rule]]` entry omits `stage`, we will parse the filename to infer it. Files that do not follow the naming rule will be rejected so users must obey the convention for custom plugins.

## Work Breakdown

1. **Inventory & Naming**
   - Enumerate all rule IDs per current script (dv_text_rules, naming_rules, etc.).
   - Define filename mapping, e.g. `flow.multiple_nonblocking.ast.py`.
   - Decide whether nested directories (flow/ or log/) are required; start with flat files for simplicity.

2. **Pilot Refactor (DV Text Rules)**
   - Extract a subset (e.g. flow.* rules) from `plugins/dv_text_rules.py` into dedicated files following the naming rule.
   - Move common helper regex/utility code into `plugins/lib/dv_helpers.py` if needed.
   - Update `sv-mint.toml` to point the selected rules at their new files (keep stage for now).
   - Run `cargo test --test cli_smoke` to ensure the new files behave identically.

3. **Automated Stage Inference**
   - Extend config loading so that if `stage` is omitted, the loader reads `<stage>` from the script filename suffix.
   - Emit a config error if the suffix is missing or invalid.
   - Add tests covering the inference logic.

4. **Complete DV Text Refactor**
   - Repeat extraction for the remaining `dv_text_rules` rules until the monolithic file is empty.
   - Delete `dv_text_rules.py` and adjust documentation (`docs/plugins/dv_text_rules.md`) to reference the new file layout.

5. **Other Plugins**
   - Apply the same approach to `naming_rules.py`, `format_spacing.py`, and other multi-rule scripts in manageable batches.
   - Keep running `cargo test --test cli_smoke` after each batch.

6. **Documentation & Samples**
   - Update README/Sample configs to describe the naming convention and stage inference.
   - Document how custom rules should be named and how stage inference works.

7. **Cleanup**
   - Remove now-unused helper code from old aggregate scripts.
   - Ensure build/test CI scripts reflect the new file structure.

## Timeline
- Day 1: Inventory, pilot extraction for a small subset, add stage inference logic.
- Day 2+: Continue refactor in batches, update docs, final cleanup.

