# dv_text_rules.py

- **Script**: `plugins/dv_text_rules.py`
- **Stage**: `raw_text`
- **Key Inputs**: `text`, `path`
- **Rules**:
  - ``style.function_scope`` (warning): Functions inside packages/modules/interfaces must be `automatic` or `static`
  - ``rand.dv_macro_required`` (warning): Enforce `DV_CHECK_*RANDOMIZE*` macros instead of raw `randomize()`
  - ``rand.dv_macro_with_required`` (warning): Require the `_WITH` DV macros when constraints are present
  - ``log.uvm_arg_macro`` (warning): `uvm_{info,error,fatal}` must use `` `gfn``/`` `gtn`` tags
  - ``log.no_uvm_warning`` (warning): Ban `uvm_warning` in favor of `uvm_error`/`uvm_fatal`
  - ``log.no_uvm_report_api`` (warning): Forbid `uvm_report_*` helpers and require the shorthand macros
  - ``log.no_display`` (warning): Forbid `$display` in DV code
  - ``log.no_none_full`` (warning): Ban `UVM_NONE` and `UVM_FULL` verbosity levels
  - ``log.allowed_verbosity`` (warning): `uvm_*` macros must use UVM_LOW/MEDIUM/HIGH/DEBUG
  - ``dpi.import_prefix`` (warning): Imported DPI symbols must start with `c_dpi_`
  - ``dpi.export_prefix`` (warning): Exported DPI handles must start with `sv_dpi_`
  - ``macro.missing_undef`` (warning): Local `` `define`` entries must be `` `undef``’d in the same file
  - ``macro.guard_required`` (warning): Macros in global `_macros.svh` headers need `` `ifndef`` guards
  - ``macro.no_local_guard`` (warning): Local macros must not use `` `ifndef`` guards
  - ``macro.dv_prefix_header_only`` (warning): `DV_*` macros belong only in shared `_macros.svh` headers
  - ``macro.module_prefix`` (warning): Module-local macros must be prefixed with the module name
  - ``flow.wait_fork_isolation`` (warning): `wait fork` must be replaced with isolation fork helpers
  - ``flow.wait_macro_required`` (warning): Raw `wait (cond)` usage must be replaced with `` `DV_WAIT``
  - ``flow.spinwait_macro_required`` (warning): `while` polling loops must live inside `` `DV_SPINWAIT``
  - ``seq.no_uvm_do`` (warning): Forbid legacy `` `uvm_do`` macros
  - ``scoreboard.dv_eot_required`` (warning): Scoreboard classes must call `DV_EOT_PRINT_*` macros
  - ``lang.no_program_construct`` (warning): Ban the `program` language construct
  - ``flow.no_fork_label`` (warning): Forbid labeled `fork : label` syntax
  - ``flow.no_disable_fork_label`` (warning): `disable fork_label` is not portable
  - ``check.dv_macro_required`` (warning): Comparison-based checks must use `DV_CHECK_*` macros

## Rule Details

### `style.function_scope`
#### Trigger
Finds `function` declarations outside classes that omit both `automatic` and `static`.
#### Message
`` function must declare automatic or static ``
#### Remediation
Add the `automatic` keyword (or `static` when intentional) so lifetime semantics are explicit.
#### Good

```systemverilog
function automatic int calc_checksum(input int data);
  return data ^ 32'hDEADBEEF;
endfunction
```

#### Bad

```systemverilog
function int calc_checksum(input int data);  // missing automatic/static
  return data;
endfunction
```

### `rand.dv_macro_required`
#### Trigger
Looks for bare `randomize()` or `std::randomize()` calls.
#### Message
`` use DV_CHECK_*RANDOMIZE* macros instead of raw randomize() ``
#### Remediation
Wrap every randomization call with `DV_CHECK_RANDOMIZE_FATAL`, `DV_CHECK_STD_RANDOMIZE_FATAL`, or `DV_CHECK_MEMBER_RANDOMIZE_FATAL`.
#### Good

```systemverilog
DV_CHECK_RANDOMIZE_FATAL(req.randomize());
```

#### Bad

```systemverilog
req.randomize();  // missing DV_CHECK_* wrapper
```

### `rand.dv_macro_with_required`
#### Trigger
Detects `randomize() with { ... }` blocks not already wrapped by `_WITH` macros.
#### Message
`` use DV_CHECK_*_WITH_FATAL macros when constraints are present ``
#### Remediation
Switch to `DV_CHECK_RANDOMIZE_WITH_FATAL`, `DV_CHECK_STD_RANDOMIZE_WITH_FATAL`, etc.
#### Good

```systemverilog
DV_CHECK_RANDOMIZE_WITH_FATAL(req.randomize() with { kind inside {READ}; });
```

#### Bad

```systemverilog
req.randomize() with { kind inside {READ}; };
```

### `log.uvm_arg_macro`
#### Trigger
Ensures the first argument to `uvm_info/error/fatal` is `` `gfn`` or `` `gtn``.
#### Message
`` first argument to uvm_* must be `gfn or `gtn ``
#### Remediation
Replace literal strings with the standard macros for hierarchy tags.
#### Good

```systemverilog
uvm_info(`gfn, "DMA started", UVM_LOW);
```

#### Bad

```systemverilog
uvm_info("dma", "DMA started", UVM_LOW);
```

### `log.no_uvm_warning`
#### Trigger
Flags any use of `uvm_warning`.
#### Message
`` uvm_warning is banned; use uvm_error or uvm_fatal ``
#### Remediation
Upgrade warnings to `uvm_error` (or `uvm_fatal` when appropriate).
#### Good

```systemverilog
uvm_error(`gfn, "Timeout waiting for ack");
```

#### Bad

```systemverilog
uvm_warning(`gfn, "Timeout waiting for ack");
```

### `log.no_uvm_report_api`
#### Trigger
Searches for `uvm_report_*` calls.
#### Message
`` use uvm_info/error/fatal instead of uvm_report_* APIs ``
#### Remediation
Switch to the shorthand macros (`uvm_info`, etc.).
#### Good

```systemverilog
uvm_info(`gfn, "Starting sequence", UVM_MEDIUM);
```

#### Bad

```systemverilog
uvm_report_info(`gfn, "Starting sequence", UVM_MEDIUM);
```

### `log.no_display`
#### Trigger
Looks for `$display` within DV sources.
#### Message
`` use uvm_* logging macros instead of $display ``
#### Remediation
Replace `$display` with `uvm_info` and friends.
#### Good

```systemverilog
uvm_info(`gfn, $sformatf("value=%0d", value_q), UVM_LOW);
```

#### Bad

```systemverilog
$display("value=%0d", value_q);
```

### `log.no_none_full`
#### Trigger
Flags verbosity arguments equal to `UVM_NONE` or `UVM_FULL`.
#### Message
`` use UVM_LOW/MEDIUM/HIGH/DEBUG verbosity levels ``
#### Remediation
Choose one of the supported verbosity constants.
#### Good

```systemverilog
uvm_info(`gfn, "Ping", UVM_LOW);
```

#### Bad

```systemverilog
uvm_info(`gfn, "Ping", UVM_NONE);
```

### `log.allowed_verbosity`
#### Trigger
Warns when the verbosity argument is a numeric literal or custom value.
#### Message
`` verbosity must be UVM_LOW/MEDIUM/HIGH/DEBUG ``
#### Remediation
Stick to the canonical verbosity constants.
#### Good

```systemverilog
uvm_info(`gfn, "Packet received", UVM_HIGH);
```

#### Bad

```systemverilog
uvm_info(`gfn, "Packet received", 700);
```

### `dpi.import_prefix`
#### Trigger
Inspects DPI import statements and verifies identifiers start with `c_dpi_`.
#### Message
`` imported DPI symbol must start with c_dpi_ ``
#### Remediation
Rename imported C functions with the `c_dpi_` prefix.
#### Good

```systemverilog
import "DPI-C" function int c_dpi_hash(input int data);
```

#### Bad

```systemverilog
import "DPI-C" function int hash(input int data);
```

### `dpi.export_prefix`
#### Trigger
Checks DPI export declarations for the `sv_dpi_` prefix.
#### Message
`` exported DPI symbol must start with sv_dpi_ ``
#### Remediation
Rename exported tasks/functions accordingly.
#### Good

```systemverilog
export "DPI-C" task sv_dpi_alert;
```

#### Bad

```systemverilog
export "DPI-C" task alert_task;
```

### `macro.missing_undef`
#### Trigger
Finds local `` `define`` statements that never see an `` `undef`` in the same file.
#### Message
`` local macro <name> must be undefined before EOF ``
#### Remediation
Add `` `undef`` once the macro is no longer needed.
#### Good

```systemverilog
`define _INC(x) ((x)+1)
assign data_o = `_INC(data_i);
`undef _INC
```

#### Bad

```systemverilog
`define _INC(x) ((x)+1)
assign data_o = `_INC(data_i);  // leaks macro
```

### `macro.guard_required`
#### Trigger
Ensures `_macros.svh` files wrap each `define` in `` `ifndef`` guards.
#### Message
`` macro headers must wrap definitions with `ifndef/`define/`endif ``
#### Remediation
Add guards so re-including the header is safe.
#### Good

```systemverilog
`ifndef FOO_MACROS_SVH
`define FOO_MACROS_SVH
`define FOO_CLR(req) req.clear()
`endif
```

#### Bad

```systemverilog
`define FOO_CLR(req) req.clear()  // unguarded in shared header
```

### `macro.no_local_guard`
#### Trigger
Warns when source files (non-header) wrap local macros inside `` `ifndef``.
#### Message
`` local macros must not use `ifndef guards ``
#### Remediation
Remove the guard so redefinition errors surface immediately.
#### Good

```systemverilog
`define _LOCAL_DEBUG(msg) \
  uvm_info(`gfn, msg, UVM_LOW)
`undef _LOCAL_DEBUG
```

#### Bad

```systemverilog
`ifndef _LOCAL_DEBUG
`define _LOCAL_DEBUG(msg) $display(msg);
`endif
```

### `macro.dv_prefix_header_only`
#### Trigger
Flags `DV_*` macros defined outside shared `_macros.svh` headers.
#### Message
`` DV_* macros must live in shared macro headers ``
#### Remediation
Move the macro into the common header or rename it without the `DV_` prefix.
#### Good

```systemverilog
// shared_macros.svh
`define DV_RAL_POKE(addr, data) \
  `uvm_info(`gfn, {"poke:", addr}, UVM_HIGH)
```

#### Bad

```systemverilog
// inside a test .sv
`define DV_RAL_POKE(addr, data) $display(addr, data);
```

### `macro.module_prefix`
#### Trigger
Ensures macros defined inside modules start with the module name in uppercase.
#### Message
`` module-local macros must be prefixed with MODULE_NAME_ ``
#### Remediation
Rename macros to `FOO_CFG_*` if they live inside `module foo`.
#### Good

```systemverilog
module foo;
  `define FOO_SET_CFG(val) cfg_q = (val)
endmodule
```

#### Bad

```systemverilog
module foo;
  `define SET_CFG(val) cfg_q = (val);  // missing FOO_ prefix
endmodule
```

### `flow.wait_fork_isolation`
#### Trigger
Reports `wait fork`.
#### Message
`` wait fork is banned; use isolation helpers ``
#### Remediation
Use watchdog-backed isolation helpers such as `DV_SPINWAIT`.
#### Good

```systemverilog
`DV_SPINWAIT(wait_done);
```

#### Bad

```systemverilog
wait fork;  // blocked until all child processes finish
```

### `flow.wait_macro_required`
#### Trigger
Detects raw `wait (cond)` statements.
#### Message
`` use `DV_WAIT(cond)` instead of raw wait ``
#### Remediation
Wrap waits with the macro so watchdog timeouts are included.
#### Good

```systemverilog
`DV_WAIT(req_done)
```

#### Bad

```systemverilog
wait (req_done);
```

### `flow.spinwait_macro_required`
#### Trigger
Flags `while` polling loops outside of `` `DV_SPINWAIT``.
#### Message
`` polling loops must use `DV_SPINWAIT``
#### Remediation
Wrap loops with the macro or move them into `DV_SPINWAIT`.
#### Good

```systemverilog
`DV_SPINWAIT(req_done)
```

#### Bad

```systemverilog
while (!req_done) begin
  #10ns;
end
```

### `flow.no_fork_label`
#### Trigger
Looks for `fork : label` syntax.
#### Message
`` fork blocks must not be labeled ``
#### Remediation
Use unlabeled `fork ... join` blocks or isolation helpers.
#### Good

```systemverilog
fork
  do_task();
join_none
```

#### Bad

```systemverilog
fork : worker_threads
  do_task();
join
```

### `flow.no_disable_fork_label`
#### Trigger
Warns when `disable` targets a fork label.
#### Message
`` disable fork_label is not portable; use disable fork ``
#### Remediation
Call `disable fork;` or rely on DV isolation helpers instead.
#### Good

```systemverilog
disable fork;
```

#### Bad

```systemverilog
disable worker_threads;
```

### `seq.no_uvm_do`
#### Trigger
Matches `` `uvm_do`` and similar legacy macros.
#### Message
`` use start_item/randomize/finish_item instead of `uvm_do ``
#### Remediation
Expand the macro into explicit sequence item handling.
#### Good

```systemverilog
req = my_item::type_id::create("req");
start_item(req);
DV_CHECK_RANDOMIZE_FATAL(req.randomize());
finish_item(req);
```

#### Bad

```systemverilog
`uvm_do(req)
```

### `scoreboard.dv_eot_required`
#### Trigger
Looks for classes ending with `_scoreboard` that never invoke `DV_EOT_PRINT_*`.
#### Message
`` scoreboard must call DV_EOT_PRINT_* macros ``
#### Remediation
Insert the macro in `report_phase` or `phase_ready_to_end`.
#### Good

```systemverilog
class my_scoreboard extends uvm_component;
  function void report_phase(uvm_phase phase);
    `DV_EOT_PRINT_SB("my_scoreboard")
  endfunction
endclass
```

#### Bad

```systemverilog
class my_scoreboard extends uvm_component;
  // no DV_EOT_PRINT_* invocation
endclass
```

### `lang.no_program_construct`
#### Trigger
Scans for the `program` keyword.
#### Message
`` program blocks are forbidden in DV sources ``
#### Remediation
Use `module`/`interface`/`class` constructs instead of `program`.
#### Good

```systemverilog
module testbench;
endmodule
```

#### Bad

```systemverilog
program automatic testbench;
endprogram
```

### `check.dv_macro_required`
#### Trigger
Finds `if (lhs != rhs) uvm_error(...)` style comparisons that omit `DV_CHECK_*`.
#### Message
`` use DV_CHECK_* macros for comparison-based checks ``
#### Remediation
Replace manual comparisons with `DV_CHECK_EQ`, `DV_CHECK_NE`, etc.
#### Good

```systemverilog
`DV_CHECK_EQ(exp_data, act_data)
```

#### Bad

```systemverilog
if (exp_data != act_data) begin
  uvm_error(`gfn, "Mismatch");
end
```
