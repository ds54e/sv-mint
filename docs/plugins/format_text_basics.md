# format_text_basics.py

- **Script**: `plugins/format_text_basics.py`
- **Stage**: `raw_text`
- **Key Inputs**: `text`
- **Rules**:
  - ``format.ascii_only`` (warning): Reject non-ASCII characters
  - ``format.no_tabs`` (warning): Reject tab characters
  - ``format.no_trailing_whitespace`` (warning): Flag trailing whitespace
  - ``format.final_newline`` (warning): Require a trailing newline

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


### `format.no_tabs`
#### Trigger
Emits a violation for every tab (`\t`) encountered.
#### Message
`` tab character detected ``
#### Remediation
Replace tabs with spaces and follow the widths enforced by `format_indent_rules`.
#### Good

```systemverilog
logic ready;
```

#### Bad

```systemverilog
	logic ready;
```

- Tabs at the start of the line shift alignment between tools.

#### Notes
Pair this with `.editorconfig` `indent_style = space`. If you absolutely must allow tabs (e.g., when linting legacy IP), disable the rule via its `[[rule]]` entry and re-enable it once the migration is complete.

### `format.no_trailing_whitespace`
#### Trigger
Reverse scans each line and flags trailing spaces or tabs.
#### Message
`` trailing whitespace at line end ``
#### Remediation
Trim on save or rely on editor hooks.
#### Good

```systemverilog
assign ready_o = valid_i;
```

#### Bad

```systemverilog
assign ready_o = valid_i;␠
```

#### Notes
sv-mint analyzes LF-normalized text, so CRLF mixes still produce correct columns. Consider the `trailing-whitespace` pre-commit hook to catch violations before CI.

### `format.final_newline`
#### Trigger
Warns when the file does not end with `\n`.
#### Message
`` file must end with newline ``
#### Remediation
Insert a newline after the last line.
#### Good

```systemverilog
module foo;
endmodule

```

#### Bad

```systemverilog
module foo;
endmodule```
#### Notes
Git adds `\ No newline at end of file` to diffs; this rule catches the issue before CI noise appears.
