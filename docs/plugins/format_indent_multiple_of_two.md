# format_indent_multiple_of_two.py

- **Script**: `plugins/format.indent_multiple_of_two.raw.py`
- **Stage**: `raw_text`
- **Key Inputs**: `text`
- **Shared Helpers**: `plugins/lib/format_indent_ruleset.py`
- **Rule**:
  - ``format.indent_multiple_of_two`` (warning): Enforce indentation in multiples of two spaces

## Rule Details

### `format.indent_multiple_of_two`
#### Trigger
Computes indentation width after stripping tabs; flags lines with an odd number of spaces.
#### Message
`` indentation should be multiples of 2 spaces ``
#### Remediation
Replace tabs with spaces and keep indentation at two-space steps.
#### Good

```systemverilog
module foo;
  logic data_q;
endmodule
```

#### Bad

```systemverilog
module foo;
   logic  data_q;  // uneven indentation
endmodule
```
