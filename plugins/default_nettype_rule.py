import re

DIRECTIVE_RE = re.compile(r"`default_nettype\s+([A-Za-z_]\w*)", re.IGNORECASE | re.MULTILINE)
PLACEMENT_LIMIT = 20


def check(req):
    if req.get("stage") != "raw_text":
        return []
    payload = req.get("payload") or {}
    text = payload.get("text") or ""
    out = []
    matches = list(DIRECTIVE_RE.finditer(text))
    if not matches:
        out.append(_violation("lang.default_nettype_missing", 0, text, "file must declare `default_nettype none` near the top"))
        return out
    first = matches[0]
    value = first.group(1).lower()
    loc = _loc(text, first.start())
    if value != "none":
        out.append({
            "rule_id": "lang.default_nettype_none",
            "severity": "warning",
            "message": "`default_nettype` must be set to `none`",
            "location": loc,
        })
    else:
        significant = _significant_lines_before(text, loc["line"])
        if significant > PLACEMENT_LIMIT:
            out.append({
                "rule_id": "lang.default_nettype_placement",
                "severity": "warning",
                "message": f"`default_nettype none` should appear within the first {PLACEMENT_LIMIT} significant lines",
                "location": loc,
            })
    out.extend(_check_trailing_reset(text))
    return out


def _loc(text, index):
    line = text.count("\n", 0, index) + 1
    prev = text.rfind("\n", 0, index)
    col = index + 1 if prev < 0 else index - prev
    return {"line": line, "col": col, "end_line": line, "end_col": col + 1}


def _violation(rule_id, index, text, message):
    return {
        "rule_id": rule_id,
        "severity": "warning",
        "message": message,
        "location": _loc(text, index),
    }


def _significant_lines_before(text, target_line):
    count = 0
    in_block = False
    for idx, line in enumerate(text.splitlines(), 1):
        if idx >= target_line:
            break
        stripped = line.strip()
        if not stripped:
            continue
        if in_block:
            if "*/" in stripped:
                in_block = False
            continue
        if stripped.startswith("/*"):
            if "*/" not in stripped:
                in_block = True
            continue
        if stripped.startswith("//"):
            continue
        count += 1
    return count


def _check_trailing_reset(text):
    out = []
    matches = list(DIRECTIVE_RE.finditer(text))
    if not matches:
        return out
    last = matches[-1]
    value = last.group(1).lower()
    loc = _loc(text, last.start())
    if value != "wire":
        out.append({
            "rule_id": "lang.default_nettype_reset",
            "severity": "warning",
            "message": "`default_nettype none` should be reset to `wire` at the end of the file",
            "location": loc,
        })
    return out
