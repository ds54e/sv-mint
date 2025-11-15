import re

POS_ARG_RE = re.compile(r"(?m)^\s*[A-Za-z_]\w*\s+[A-Za-z_]\w*\s*\((?!\s*\.)")


def locate(text, index):
    line = text.count("\n", 0, index) + 1
    prev = text.rfind("\n", 0, index)
    col = index + 1 if prev < 0 else index - prev
    return {
        "line": line,
        "col": col,
        "end_line": line,
        "end_col": col + 1,
    }


def check(req):
    if req.get("stage") != "raw_text":
        return []
    payload = req.get("payload") or {}
    text = payload.get("text") or ""
    out = []
    for match in POS_ARG_RE.finditer(text):
        out.append({
            "rule_id": "module.named_ports_required",
            "severity": "warning",
            "message": "use named port connections instead of positional arguments",
            "location": locate(text, match.start()),
        })
    return out
