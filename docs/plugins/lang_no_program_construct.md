# lang_no_program_construct.py

- **Script**: `plugins/lang.no_program_construct.raw.py`
- **Stage**: `raw_text`
- **Key Inputs**: `text`, `path`
- **Shared Helpers**: `plugins/lib/dv_text_ruleset.py`
- **Rule**:
  - ``lang.no_program_construct`` (warning): Ban the `program` language construct

## Rule Details

### `lang.no_program_construct`
#### Trigger
Scans for the `program` keyword.
#### Message
`` program blocks are forbidden in DV sources ``
#### Remediation
Use `module`/`interface`/`class` constructs instead of `program`.
#### Good

```systemverilog
module testbench;
endmodule
```

#### Bad

```systemverilog
program automatic testbench;
endprogram
```
