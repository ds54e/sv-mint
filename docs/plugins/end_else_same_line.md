# format.end_else_inline.pp.py

- **Stage**: `pp_text`
- **Key Inputs**: Preprocessed `text`
- **Summary**: Require `else` to share the same line as the preceding `end`

## Details

### Trigger
Detects the pattern `end` + whitespace + newline + whitespace + `else` and reports the `else` location.
### Message
`` else must be on the same line as the preceding end ``
### Remediation
Join `end else` onto a single line or adopt `end else begin` formatting consistently.
### Notes
Lines split by comments are ignored. The goal is to keep `end/else` visually paired.
### Good

```systemverilog
if (req_i) begin
  data_q <= data_d;
end else begin
  data_q <= '0;
end
```

### Bad

```systemverilog
if (req_i) begin
  data_q <= data_d;
end
else begin
  data_q <= '0;
end
```

### Additional Tips
When `end` has a trailing comment (`end // state latch`), leave enough spacing so `end // state latch else ...` reads clearly. Because the rule scans `pp_text`, `else` guarded by `ifdef` blocks may be skipped if not present after preprocessing.
