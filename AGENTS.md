# Repository Guidelines

## Instructions for Agents
- When users ask questions or request explanations, respond in Japanese.
- Do not add new comments or modify existing ones when editing code.
- Write any documentation updates in English.
- When tests fail, do not change rule fixtures unless explicitly requested; investigate and report the cause instead.
- Do not modify fixtures unless the user explicitly asks you to; the user will adjust fixtures themselves.

## Project Layout
`src/lib.rs` wires the Rust core, while `src/bin/sv-mint.rs` exposes the CLI. Pipeline logic lives under `src/core/` (types, size guards, line maps); diagnostics and logging under `src/diag/`; SystemVerilog artifacts under `src/sv/`. IO helpers (config parsing, output formatting) live in `src/io/`; Python plugins under `plugins/`; shared helpers under `plugins/lib/`. Defaults are in `sv-mint.toml`, and Rust formatting is configured via `rustfmt.toml`.

## Build, Test, and Development Commands
- `cargo build --release`: produce `target/release/sv-mint` for distribution.
- `cargo check`: fast validation during development.
- `cargo fmt --all`: enforce Rust formatting before review.
- `cargo clippy --all-targets --all-features`: lint for common mistakes (treat warnings as blockers).
- `sv-mint --config ./sv-mint.toml path/to/file.sv`: run the CLI on sample files and adjust plugin scripts in `sv-mint.toml` as needed.

## Coding Style and Naming
Rust code uses 4-space indent, snake_case modules, UpperCamelCase types, and concise doc comments. Keep functions short and favor explicit enums/structs over ad-hoc tuples. Python plugins follow PEP 8 with lowercase module names and descriptive helpers. Run `cargo fmt` before committing, align Python imports, and wrap long JSON literals for readability.

## Testing Guidelines
There is no dedicated suite yet; add unit tests near their modules (e.g., `src/core/pipeline.rs` → `tests/pipeline.rs` or inline `#[cfg(test)]`). Use `cargo test --lib` for Rust logic and standalone `pytest` for utilities under `plugins/lib/`. Pick descriptive names such as `handles_inlined_cst_payload`, and document edge cases like size-guard thresholds or plugin timeouts. Share failing SV fixtures under `fixtures/`.

## Commit and PR Guidelines
History favors short imperative commits (`Add inline CST IR`). Keep that style, scope each commit to a single concern, and provide rationale when touching parser interfaces. PRs should summarize behavioral impact, list affected stages/plugins, mention config changes, and link related issues. Include reproduction commands (`cargo run -- …`) and, when UI/log output changes, attach stderr samples for reviewers.

## Security and Configuration Notes
Respect the thresholds baked into `core/size_guard.rs` and document any changes in the PR description. Validate input paths when adding plugins and avoid shell interpolation. Override `sv-mint.toml` locally to keep sensitive include paths or defines outside the repo.
