# flow_no_disable_fork_label.py

- **Script**: `plugins/flow.no_disable_fork_label.raw.py`
- **Stage**: `raw_text`
- **Key Inputs**: `text`, `path`
- **Shared Helpers**: `plugins/lib/dv_text_ruleset.py`
- **Rule**:
  - ``flow.no_disable_fork_label`` (warning): `disable fork_label` is not portable

## Rule Details

### `flow.no_disable_fork_label`
#### Trigger
Warns when `disable` targets a fork label.
#### Message
`` disable fork_label is not portable; use disable fork ``
#### Remediation
Call `disable fork;` or rely on DV isolation helpers instead.
#### Good

```systemverilog
disable fork;
```

#### Bad

```systemverilog
disable worker_threads;
```
