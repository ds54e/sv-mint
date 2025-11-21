import re

from lib.dv_helpers import loc, raw_text_inputs


def check(req):
    if req.get("stage") != "raw_text":
        return []
    inputs = raw_text_inputs(req)
    if not inputs:
        return []
    text, path = inputs
    macros_file = path.endswith("_macros.svh")
    defines = list(re.finditer(r"(?m)^\s*`define\s+([A-Za-z_]\w*)", text))
    undefs = {m.group(1) for m in re.finditer(r"`undef\s+([A-Za-z_]\w*)", text)}
    out = []
    if not macros_file:
        for match in defines:
            name = match.group(1)
            if name in undefs:
                continue
            out.append({
                "rule_id": "macros_close_with_undef",
                "severity": "warning",
                "message": f"`define {name} must be undefined at end of file",
                "location": loc(text, match.start(1)),
            })
    return out
