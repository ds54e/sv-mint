import re

UNSIZED_BASE = re.compile(r"(?<![0-9_])'(?:[bBdDhHoO])")


def locate(text, index):
    line = text.count("\n", 0, index) + 1
    prev = text.rfind("\n", 0, index)
    col = index + 1 if prev < 0 else index - prev
    return {"line": line, "col": col, "end_line": line, "end_col": col + 1}


def check(req):
    if req.get("stage") != "raw_text":
        return []
    payload = req.get("payload") or {}
    text = payload.get("text") or ""
    out = []
    for match in UNSIZED_BASE.finditer(text):
        loc = locate(text, match.start())
        out.append({
            "rule_id": "width.unsized_base_literal",
            "severity": "warning",
            "message": "base literal must include explicit width (e.g. 8'hFF)",
            "location": loc,
        })
    return out
