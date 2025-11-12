PREPROC = ("`define", "`ifdef", "`ifndef", "`elsif", "`else", "`endif")


def check(req):
    if req.get("stage") != "raw_text":
        return []
    payload = req.get("payload") or {}
    text = payload.get("text") or ""
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
    return out


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
