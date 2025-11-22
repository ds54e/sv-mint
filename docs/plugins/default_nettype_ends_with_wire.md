# default_nettype_ends_with_wire

## Script
- `plugins/default_nettype_ends_with_wire.cst.py`

## Description
- Files must reset `default_nettype` back to `wire` near the end

## Good

```systemverilog
`default_nettype none
module m;
endmodule
`default_nettype wire
```

```systemverilog
module m;
endmodule
```

## Bad

```systemverilog
`default_nettype none

module m;
endmodule
```
