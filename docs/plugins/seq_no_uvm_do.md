# seq_no_uvm_do.py

- **Script**: `plugins/seq.no_uvm_do.raw.py`
- **Stage**: `raw_text`
- **Key Inputs**: `text`, `path`
- **Shared Helpers**: `plugins/lib/dv_text_ruleset.py`
- **Rule**:
  - ``seq.no_uvm_do`` (warning): Forbid legacy `` `uvm_do`` macros

## Rule Details

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
