# sv-mint

sv-mint is a SystemVerilog lint pipeline that combines a Rust core with Python plugins. It shines when you want to add rules quickly without recompiling and target different pipeline stages (raw, preprocessed, CST, AST) as needed.

## Getting Started
1. Download the latest release from GitHub (`sv-mint-vX.Y.Z-<platform>.tar.gz`/`.zip`) and extract it somewhere on your machine.
2. Add the extracted directory (it contains `sv-mint`, `plugins/`, `sv-mint.toml`, `LICENSE`, `CHANGELOG.md`, and this `README.md`) to your `PATH`, or call the binary via an absolute path.
3. Lint your sources:
   ```bash
   ./sv-mint --config ./sv-mint.toml path/to/files/foo.sv path/to/files/bar.sv
   ```
4. Tailor rules by editing `sv-mint.toml`. Declare your `[[rule]]` entries, point `[plugin]` at your Python interpreter, and let sv-mint’s built-in defaults cover everything else unless you need overrides.

### Sample `sv-mint.toml`

```toml
[[rule]]
id = "macro_names_uppercase"

[[rule]]
id = "vars_not_left_unused"
```

5. Narrow or relax checks directly from the CLI when experimenting:
   - `sv-mint --only rule_x path/to/file.sv` runs only `rule_x`, temporarily disabling every other rule.
   - `sv-mint --disable rule_a,rule_b path/to/file.sv` disables just the listed rules; specify multiple IDs or repeat `--disable` as needed.
   - When `--only` is present, any `--disable` that follows removes rules from that already-filtered set, and referencing a nonexistent `rule_id` raises an error.

## Provenance and License
- Rust dependencies follow MIT or Apache-2.0 licenses as declared in `Cargo.toml`.
- sv-mint itself is distributed under the same terms as the repository license (see `LICENSE`).
