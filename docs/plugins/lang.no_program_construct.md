# lang.no_program_construct

- **Script**: `plugins/lang.no_program_construct.py`
- **Stage**: `raw_text`
- **Key Inputs**: `text`, `path`
- **Shared Helpers**: `plugins/lib/dv_text_ruleset.py`
- **Summary**: Ban the `program` language construct

## Details

### Trigger
Scans for the `program` keyword.
### Message
`` program blocks are forbidden in DV sources ``
### Remediation
Use `module`/`interface`/`class` constructs instead of `program`.
### Good

```systemverilog
module testbench;
endmodule
```

### Bad

```systemverilog
program automatic testbench;
endprogram
```
