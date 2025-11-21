import re

DEFINE_RE = re.compile(r"(?m)^\s*`define\s+([A-Za-z_]\w*)")
ALL_CAPS = re.compile(r"^[A-Z][A-Z0-9_]*$")


def check(req):
    if req.get("stage") != "raw_text":
        return []
    payload = req.get("payload") or {}
    text = payload.get("text") or ""
    out = []
    for m in DEFINE_RE.finditer(text):
        name = m.group(1)
        if not ALL_CAPS.match(name):
            out.append({
                "rule_id": "macro.define_upper",
                "severity": "warning",
                "message": f"`define {name} should use ALL_CAPS",
                "location": _loc(text, m.start(1)),
            })
    return out


def _loc(text, index):
    line = text.count("\n", 0, index) + 1
    prev = text.rfind("\n", 0, index)
    col = index + 1 if prev < 0 else index - prev
    return {
        "line": line,
        "col": col,
        "end_line": line,
        "end_col": col + 1,
    }
