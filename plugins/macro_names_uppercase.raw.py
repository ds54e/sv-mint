import re

from lib.utf8 import line_starts, point_to_loc

DEFINE_RE = re.compile(r"(?m)^\s*`define\s+([A-Za-z_]\w*)")
ALL_CAPS = re.compile(r"^[A-Z][A-Z0-9_]*$")

def check(req):
    if req.get("stage") != "raw_text":
        return []
    payload = req.get("payload") or {}
    text = payload.get("text") or ""
    starts = line_starts(text)
    out = []
    for m in DEFINE_RE.finditer(text):
        name = m.group(1)
        if not ALL_CAPS.match(name):
            out.append(
                {
                    "rule_id": "macro_names_uppercase",
                    "severity": "warning",
                    "message": f"`define {name} should use ALL_CAPS",
                    "location": point_to_loc(text, m.start(1), len(name), starts),
                }
            )
    return out
