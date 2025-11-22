# macros_not_unused

- **Script**: `plugins/macros_not_unused.raw.py`
- **Stage**: `raw_text`
- **Summary**: Warn when a macro is defined but never used

## Details

### Limitations
- A comment containing the words `used` or `reserved` (case-insensitive) in the macro definition block suppresses this warning.
- For multi-line macros, place the comment on the final line without a trailing backslash to ensure it is detected.

### Good

```systemverilog
module m;

  `define MACRO_A(a) a
  `define MACRO_B(b) b // reserved
  `define MACRO_C(c) \
    if (c) begin \
      $display(1); \
    end else else \
      $display(0); \
    end // will be used later

  wire y = `MACRO_A(1'b1);

endmodule
```

### Bad

```systemverilog
module m;

  `define MACRO_A(a) a
  `define MACRO_B(b) b
  `define MACRO_C(c) \
    if (c) begin \
      $display(1); \
    end else else \
      $display(0); \
    end

endmodule
```
