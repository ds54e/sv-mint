# format_ascii_only.py

- **Script**: `plugins/format.ascii_only.raw.py`
- **Stage**: `raw_text`
- **Key Inputs**: `text`
- **Shared Helpers**: `plugins/lib/format_text_ruleset.py`
- **Rule**:
  - ``format.ascii_only`` (warning): Reject non-ASCII characters

## Rule Details

### `format.ascii_only`
#### Trigger
Reports every character whose `ord(ch) > 127`.
#### Message
`` non-ASCII character detected ``
#### Remediation
Remove non-ASCII glyphs (comments included) or disable the rule if UTF-8 text is unavoidable.
#### Good

```systemverilog
// state machine controls DMA start
```

#### Bad

```systemverilog
// Δ-state start  ← contains non-ASCII character
```
