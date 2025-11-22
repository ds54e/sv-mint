# default_nettype_begins_with_none

## Script
- `plugins/default_nettype_begins_with_none.cst.py`

## Description
- Require `` `default_nettype none`` in every file

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
