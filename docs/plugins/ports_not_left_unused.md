# ports_not_left_unused

## Script
- `plugins/ports_not_left_unused.ast.py`

## Description
- Warn when module ports are never read or written inside the module body.
- Implicit `.*` connections are not elaborated; they will be counted as unused.
- Implicit named port shorthand (e.g., `.foo`) is elaborated and counted as a use.
- If the declaration line contains a comment with the words `used` or `reserved` (case-insensitive), the warning is suppressed.
- Why: Unused ports suggest missing connections or obsolete interfaces.
## Good

```systemverilog
module m (
  input logic a,
  input logic b1, // reserved
  input logic b2, // used
  output logic c
);
  assign c = a;
endmodule
```

## Bad

```systemverilog
module m (
  input logic a,
  output logic b
);
  assign b = 1'b1;
endmodule
```
