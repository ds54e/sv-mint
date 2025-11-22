import re

from lib.utf8 import line_starts, point_to_loc

def check(req):
    if req.get("stage") != "raw_text":
        return []
    payload = req.get("payload") or {}
    text = payload.get("text") or ""
    path = req.get("path") or ""
    macros_file = path.endswith("_macros.svh")
    starts = line_starts(text)
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
                    "location": point_to_loc(text, match.start(1), len(name), starts),
                }
            )
    return out
