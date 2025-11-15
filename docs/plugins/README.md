# Plugin Documentation

This directory contains one Markdown file per rule. Each file is named after the rule ID (`<rule_id>.md`) and documents:

- The script path under `plugins/`
- The stage payload it consumes (`raw_text`, `pp_text`, `cst`, or `ast`)
- A short summary plus trigger/message/remediation examples

When adding or updating rules:

1. Implement the script using the `<rule_id>.<stage>.py` convention under `plugins/`.
2. Create or update `docs/plugins/<rule_id>.md` so it mirrors the script behavior.
3. Keep the headings and bullet layout consistent with existing files so the docs stay easy to scan.

For plugin configuration, loader behavior, and example `sv-mint.toml` entries, follow the instructions in the top-level `README.md`. This file focuses solely on how the per-rule documentation set is organized.
