# Full CST IR Expansion and Plugin Migration Plan

## Goals
- Expose full sv-parser CST data plus precomputed helpers to plugins.
- Simplify existing CST rules by replacing ad-hoc token scans with structured fields.
- Drop backward compatibility with the previous lightweight CST payload.
- Keep `source_text`/token text; `pp_text` remains only for compatibility.

## Schema Extensions (cst_ir)
- Schema version: bump to v2 (breaking change).
- Token records: keep start/end/kind/text.
- Add `tok_kind_map` (name -> id) to avoid per-plugin map building.
- Add `source_text` (already present) and keep `pp_text` as legacy.
- Function/Task:
  - `return_type` field pointing to DataType/NetType/ImplicitDataType/VoidType node id.
  - `ports` array: entries with `{dir, type: node_id, name_token: token_id, expr: node_id?}`; `ImplicitDataType` must be emitted for implicit types.
- Parameter/Localparam:
  - `type` field with node id; emit `ImplicitDataType` when type is implicit.
- Sensitivity lists:
  - `AlwaysConstruct` includes events array (edge, expr id, separator kind: comma/or).
  - `always_kind` field (ff/comb/latch/bare) or keyword token id.
- Directives:
  - Collected list of directives (`default_nettype`, `define`, `undef`, `include`, `ifdef/ifndef` etc.) with value and location.
  - Include tree info for includes/ifdef nesting (if feasible in this pass).
- Case statements:
  - Flags `has_default`, `is_unique`, `is_priority` and item labels summary.
- Instances:
  - Connection list with named/positional flag, port name, expr node id.
- Declarations:
  - For ports/nets/vars/params: declaration kind and name token id; optional scope id.
- References/Scopes (if time permits):
  - Symbol table entries with definition node id and reference node ids.
- Attributes/Comments (optional if time permits):
  - Collected list with location and text.

## Code Changes (Rust)
- Update `sv::cst_ir` builder to emit schema v2 and all new fields.
- Extend node serialization with `fields`/child links for the above data.
- Add `tok_kind_map` to `cst_ir`.
- Update payload serialization (no mode), keep `source_text`.
- Enhance `Cst` helper to expose new fields and tok_kind lookup.

## Plugin Migration
- Update all CST plugins to rely on structured fields:
  - `functions_have_explicit_types`: use return_type/ports types.
  - `parameter_has_type`: use param type field (ImplicitDataType detection).
  - `sensitivity_list_uses_commas`: use events separator flag.
  - `always_is_structured`, `always_ff_uses_nonblocking`, `always_comb_uses_blocking`: use `always_kind` and sensitivity events.
  - `default_nettype_*`: use directive list.
  - `case_has_default_branch`: use `has_default` flag.
  - `instances_use_named_ports`: use connection structure.
  - Naming/unused rules: leverage declaration/refs if available.
- Remove token-scan fallbacks; assume full CST v2.

## Tests
- Update/extend CLI smoke fixtures if needed for new structured checks.
- Add unit/snapshot tests for cst_ir schema v2 (Rust side) to validate presence of new fields.
- Run `cargo test --test cli_smoke -- --nocapture` and other relevant suites.

## Migration/Compatibility
- Breaking change: only schema v2 is supported; old payload/mode removed.
- Document schema bump and new fields for plugin authors.

## Execution Steps
1. Implement schema v2 in Rust (`cst_ir` builder) with new fields.
2. Update `Cst` helper and tok_kind lookup utilities.
3. Migrate CST plugins to structured fields; delete fallbacks.
4. Update/add tests (Rust schema checks + CLI smoke).
5. Run full test suite; fix regressions.
6. Document the change (README/AGENTS if needed) and push.
