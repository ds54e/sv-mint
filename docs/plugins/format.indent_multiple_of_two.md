# format.indent_multiple_of_two

- **Stage**: `raw_text`
- **Key Inputs**: `text`
- **Shared Helpers**: `plugins/lib/format_indent_ruleset.py`
- **Summary**: Enforce indentation in multiples of two spaces

## Details

### Trigger
Computes indentation width after stripping tabs; flags lines with an odd number of spaces.
### Message
`` indentation should be multiples of 2 spaces ``
### Remediation
Replace tabs with spaces and keep indentation at two-space steps.
### Good

```systemverilog
module foo;
  logic data_q;
endmodule
```

### Bad

```systemverilog
module foo;
   logic  data_q;  // uneven indentation
endmodule
```
