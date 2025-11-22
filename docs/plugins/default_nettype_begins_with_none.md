# default_nettype_begins_with_none

## Script
- `plugins/default_nettype_begins_with_none.cst.py`

## Description
- Require `` `default_nettype none`` in every file
- Why: Forcing explicit net declarations catches typos instead of silently creating implicit wires.
## Good

```systemverilog
`default_nettype none
module m;
endmodule
`default_nettype wire
```

## Bad

```systemverilog
module m;
endmodule
```
