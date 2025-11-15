# naming.pipeline_sequence.ast.py

- **Stage**: `ast`
- **Key Inputs**: `decls`, `symbols`, `ports`
- **Shared Helpers**: `plugins/lib/naming_ruleset.py`
- **Summary**: `_q2`+ stages require preceding `_q<n-1>`

## Details

### Trigger
Ensures `_q2` and above have contiguous predecessor stages.
### Message
`` pipeline stage <name> missing previous stage `` 
### Remediation
Declare `_q`, `_q1`, `_q2`, etc., without skipping counts.
### Good

```systemverilog
logic state_q;
logic state_q1;
logic state_q2;
```

### Bad

```systemverilog
logic state_q;
logic state_q2;  // q1 missing
```
