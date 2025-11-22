# disable_targets_fork_only

- **Script**: `plugins/disable_targets_fork_only.cst.py`
- **Stage**: `cst`
- **Key Inputs**: `cst_ir.tokens`, `line_starts`, `pp_text`
- **Summary**: `disable fork_label` is not portable

## Details

### Message
`` disable block label is not portable; use disable fork ``
### Good

```systemverilog
module m;

  initial begin
    fork
      begin #1; end
      begin #2; end
    join_any
    disable fork;
  end

endmodule
```

### Bad

```systemverilog
module m;

  initial begin
    fork : fork_label
      begin #1; end
      begin #2; end
    join_any
    disable fork_label;
  end

endmodule
```
