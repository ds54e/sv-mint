# dv_text_rules.py

- **Script**: `plugins/dv_text_rules.py`
- **Stage**: `raw_text`
- **Key Inputs**: `text`, `path`
- **Rules**:
  | Rule ID | Severity | Summary |
  | --- | --- | --- |
  | `style.function_scope` | warning | Functions inside packages/modules/interfaces must be `automatic` or `static` |
  | `rand.dv_macro_required` | warning | Enforce `DV_CHECK_*RANDOMIZE*` macros instead of raw `randomize()` |
  | `log.uvm_arg_macro` | warning | `uvm_{info,error,fatal}` must use `` `gfn``/`` `gtn`` tags |
  | `log.no_uvm_warning` | warning | Ban `uvm_warning` in favor of `uvm_error`/`uvm_fatal` |
  | `log.no_uvm_report_api` | warning | Forbid `uvm_report_*` helpers and require the shorthand macros |
  | `log.no_display` | warning | Forbid `$display` in DV code |
  | `log.no_none_full` | warning | Ban `UVM_NONE` and `UVM_FULL` verbosity levels |
  | `dpi.import_prefix` | warning | Imported DPI symbols must start with `c_dpi_` |
  | `dpi.export_prefix` | warning | Exported DPI handles must start with `sv_dpi_` |
  | `macro.missing_undef` | warning | Local `` `define`` entries must be `` `undef``’d in the same file |
  | `macro.guard_required` | warning | Macros in global `_macros.svh` headers need `` `ifndef`` guards |
  | `macro.no_local_guard` | warning | Local macros must not use `` `ifndef`` guards |
  | `flow.wait_fork_isolation` | warning | `wait fork` must be replaced with isolation fork helpers |
  | `flow.wait_macro_required` | warning | Raw `wait (cond)` usage must be replaced with `` `DV_WAIT`` |
  | `flow.spinwait_macro_required` | warning | `while` polling loops must live inside `` `DV_SPINWAIT`` |
  | `seq.no_uvm_do` | warning | Forbid legacy `` `uvm_do`` macros |

## Rule Details

### `style.function_scope`
The DVCodingStyle guide requires package-level and other static functions to declare either `automatic` or `static`. This rule scans the raw text (outside of classes) and flags `function` declarations that omit both keywords, helping enforce deterministic lifetime semantics.

### `rand.dv_macro_required`
Randomization must go through `DV_CHECK_RANDOMIZE_FATAL`, `DV_CHECK_STD_RANDOMIZE_FATAL`, or `DV_CHECK_MEMBER_RANDOMIZE_FATAL`. Any direct `randomize()` or `std::randomize()` call produces a violation so DV code always benefits from the macro-provided error checks.

### Logging Rules (`log.*`)
- `log.uvm_arg_macro` ensures the first argument to `uvm_info`, `uvm_error`, and `uvm_fatal` is either `` `gfn`` or `` `gtn``. This keeps logs searchable by hierarchy, as prescribed in the logging section of DVCodingStyle.
- `log.no_uvm_warning` bans `uvm_warning` outright; the guide mandates `uvm_error` or `uvm_fatal` instead.
- `log.no_uvm_report_api` blocks `uvm_report_*` usage so teams always rely on the concise shorthand macros.
- `log.no_display` forbids `$display` and pushes users toward the UVM reporting macros.
- `log.no_none_full` prevents `UVM_NONE` and `UVM_FULL` verbosity levels, matching the recommended verbosity banding.

### DPI Rules (`dpi.*`)
Section “DPI and C Connections” requires imported functions to be prefixed with `c_dpi_` and exported handles with `sv_dpi_`. These rules look for `import "DPI"` / `export "DPI"` statements and flag any identifiers that break the prefix contract.

### `macro.missing_undef`
The macro section of the guide states that local macros must be undefined at the end of the file to avoid polluting downstream compilation units. This rule records each `` `define`` outside `_macros.svh` files and reports the ones that are never `` `undef``’d.

### `macro.guard_required`
Global macro headers (`*_macros.svh`) must wrap each definition in a `` `ifndef` guard so the macro can be safely included multiple times. Any guarded macro missing its matching `` `ifndef`` is flagged.

### `macro.no_local_guard`
Conversely, local macros (defined directly inside `.sv` or package sources) must *not* use `` `ifndef`` guards. Guards hide redefinition errors and were explicitly called out in the guide. This rule warns when a non-header source gate its macro with `` `ifndef`.

### Wait/Fork Rules (`flow.*`)
- `flow.wait_fork_isolation` rejects `wait fork`, nudging engineers toward the isolation-fork pattern (`DV_SPINWAIT`) recommended by the spec.
- `flow.wait_macro_required` warns on `wait (condition)` so that watchdog-backed `` `DV_WAIT`` helpers are used instead.
- `flow.spinwait_macro_required` reports polling `while` loops that are not invoked through `` `DV_SPINWAIT``, ensuring watchdog timers accompany non-forever loops.

### `seq.no_uvm_do`
The macro section explicitly bans `` `uvm_do`` helpers; tests must call `start_item`, randomize, `finish_item`, and `get_response` manually. This rule catches any usage of the legacy macro family so authors migrate to the recommended flow.
