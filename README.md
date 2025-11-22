# sv-mint

sv-mint is a SystemVerilog lint pipeline that combines a Rust core with Python plugins. It shines when you want to add rules quickly without recompiling and target different pipeline stages (raw, preprocessed, CST, AST) as needed.

## Getting Started
1. Download the latest release from GitHub (`sv-mint-vX.Y.Z-<platform>.tar.gz`/`.zip`) and extract it somewhere on your machine.
2. Add the extracted directory to your `PATH`, or call the binary via an absolute path.
3. Lint your sources:
   ```bash
   ./sv-mint --config ./sv-mint.toml path/to/files/foo.sv path/to/files/bar.sv
   ```
### Sample `sv-mint.toml`

```toml
[[rule]]
id = "macro_names_uppercase"

[[rule]]
id = "vars_not_left_unused"
```

## Provenance and License
- Rust dependencies follow MIT or Apache-2.0 licenses as declared in `Cargo.toml`.
- sv-mint itself is distributed under the MIT License (see `LICENSE`).
