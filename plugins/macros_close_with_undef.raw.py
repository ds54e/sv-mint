import re

def check(req):
    if req.get("stage") != "raw_text":
        return []
    payload = req.get("payload") or {}
    text = payload.get("text") or ""
    path = req.get("path") or ""
    macros_file = path.endswith("_macros.svh")
    defines = list(re.finditer(r"(?m)^\s*`define\s+([A-Za-z_]\w*)", text))
    undefs = {m.group(1) for m in re.finditer(r"`undef\s+([A-Za-z_]\w*)", text)}
    out = []
    if not macros_file:
        for match in defines:
            name = match.group(1)
            if name in undefs:
                continue
            out.append(
                {
                    "rule_id": "macros_close_with_undef",
                    "severity": "warning",
                    "message": f"`define {name} must be undefined at end of file",
                    "location": _byte_loc(text, match.start(1)),
                }
            )
    return out

def _byte_loc(text, index):
    line = text.count("\n", 0, index) + 1
    prev = text.rfind("\n", 0, index)
    col = index + 1 if prev < 0 else index - prev
    return {"line": line, "col": col, "end_line": line, "end_col": col + 1}
