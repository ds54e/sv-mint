from lib.dv_helpers import raw_text_inputs

PREPROC = ("`define", "`ifdef", "`ifndef", "`elsif", "`else", "`endif")
CACHE_KEY = "__format_indent_ruleset"


def violations_for(req, rule_id):
    table = evaluate(req)
    return list(table.get(rule_id) or [])


def evaluate(req):
    cached = req.get(CACHE_KEY)
    if cached is not None:
        return cached
    inputs = raw_text_inputs(req)
    if not inputs:
        req[CACHE_KEY] = {}
        return req[CACHE_KEY]
    text, _ = inputs
    out = []
    lines = text.splitlines()
    for idx, line in enumerate(lines, start=1):
        stripped = line.lstrip(" ")
        indent = len(line) - len(stripped)
        if stripped and indent % 2 != 0:
            out.append(_violation("format.indent_multiple_of_two", idx, indent + 1, "indentation should be multiples of 2 spaces"))
        stripped_tab = line.lstrip(" \t")
        if stripped_tab.startswith(tuple(PREPROC)) and (len(line) - len(stripped_tab)) != 0:
            out.append(_violation("format.preproc_left_align", idx, 1, "preprocessor directives must be left aligned"))
        if line.rstrip().endswith("\\") and not line.endswith("\\"):
            col = len(line.rstrip())
            out.append(_violation("format.line_continuation_right", idx, col, "line continuation \\\\ must be last character"))
    table = {}
    for item in out:
        table.setdefault(item["rule_id"], []).append(item)
    req[CACHE_KEY] = table
    return table


def _violation(rule_id, line, col, message):
    return {
        "rule_id": rule_id,
        "severity": "warning",
        "message": message,
        "location": {
            "line": line,
            "col": col,
            "end_line": line,
            "end_col": col,
        },
    }
