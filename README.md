# sv-mint

sv-mint is a SystemVerilog linting pipeline that integrates a Rust core with Python plugins. It allows for rapid rule extension without recompilation and supports analysis across multiple pipeline stages, including raw text, preprocessed output, CST, and AST.

## Getting Started
1. Download the latest release from GitHub and extract the archive to a directory.
2. Execute the linter with your configuration:
   ```bash
   sv-mint --config sv-mint.toml path/to/foo.sv path/to/bar.sv
   ```
3. For comprehensive documentation, please refer to https://deepwiki.com/ds54e/sv-mint.
   
## Provenance and License
- **Dependencies:** Rust dependencies strictly follow MIT or Apache-2.0 licenses as declared in `Cargo.toml`.
- **Distribution:** sv-mint is distributed under the MIT License.
