# Plugin Layout Notes

Every bundled rule now lives in its own script following the `<rule_id>.<stage>.py` convention where `<stage>` is one of `raw`, `pp`, `cst`, or `ast`. When you keep custom plugins under `[plugin].root` (or any directory listed in `[plugin].search_paths`), `sv-mint` can infer both the script path and stage automatically. That means your `[[rule]]` entries only need to specify `id = "rule.id"` unless you deliberately want to override the script path.

Example minimal configuration:

```toml
[plugin]
cmd = "python3"
args = ["-u", "-B"]
root = "plugins"

[[rule]]
id = "format.no_tabs"           # uses plugins/format.no_tabs.raw.py

[[rule]]
id = "naming.port_suffix"       # uses plugins/naming.port_suffix.ast.py

[[rule]]
id = "module.no_port_wildcard"  # uses plugins/module.no_port_wildcard.cst.py
```

If you want to test a local experimental script, simply override the `script` field:

```toml
[[rule]]
id = "format.no_tabs"
script = "local_experiments/no_tabs_experiment.raw.py"
```

The resolver falls back to the explicit script path whenever it is provided. This keeps the default configuration terse while still allowing per-rule overrides when needed.
