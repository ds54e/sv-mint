# scoreboard.dv_eot_required

- **Stage**: `raw_text`
- **Key Inputs**: `text`, `path`
- **Shared Helpers**: `plugins/lib/dv_text_ruleset.py`
- **Summary**: Scoreboard classes must call `DV_EOT_PRINT_*` macros

## Details

### Trigger
Looks for classes ending with `_scoreboard` that never invoke `DV_EOT_PRINT_*`.
### Message
`` scoreboard must call DV_EOT_PRINT_* macros ``
### Remediation
Insert the macro in `report_phase` or `phase_ready_to_end`.
### Good

```systemverilog
class my_scoreboard extends uvm_component;
  function void report_phase(uvm_phase phase);
    `DV_EOT_PRINT_SB("my_scoreboard")
  endfunction
endclass
```

### Bad

```systemverilog
class my_scoreboard extends uvm_component;
  // no DV_EOT_PRINT_* invocation
endclass
```
