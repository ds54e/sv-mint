import re

DEFINE_PATTERN = re.compile(r"(?m)^\s*`define\s+([A-Za-z_]\w*)")
UNDEF_PATTERN = re.compile(r"(?m)^\s*`undef\s+([A-Za-z_]\w*)")


def locate(text, index, length):
    line = text.count("\n", 0, index) + 1
    prev = text.rfind("\n", 0, index)
    col = index + 1 if prev < 0 else index - prev
    return {
        "line": line,
        "col": col,
        "end_line": line,
        "end_col": col + length - 1,
    }


def check(req):
    if req.get("stage") != "raw_text":
        return []
    payload = req.get("payload") or {}
    text = payload.get("text") or ""
    undef_names = {m.group(1) for m in UNDEF_PATTERN.finditer(text)}
    out = []
    for match in DEFINE_PATTERN.finditer(text):
        name = match.group(1)
        loc = locate(text, match.start(), len("`define"))
        if name.startswith("_"):
            if name not in undef_names:
                out.append({
                    "rule_id": "global.local_define_undef",
                    "severity": "warning",
                    "message": f"local macro {name} must be undefined after use",
                    "location": loc,
                })
        else:
            out.append({
                "rule_id": "global.prefer_parameters",
                "severity": "warning",
                "message": f"use parameters instead of global macro `{name}`",
                "location": loc,
            })
    return out
