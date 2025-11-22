# disable_targets_fork_only

## Script
- `plugins/disable_targets_fork_only.cst.py`

## Description
- `disable fork_label` is not portable

## Good

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

## Bad

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
