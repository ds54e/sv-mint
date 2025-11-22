import re

from lib.cst_inline import byte_span_to_loc

TYPEDEF_ENUM_RE = re.compile(r"typedef\s+enum(?P<head>[\s\S]*?)\{(?P<body>[\s\S]*?)\}\s*(?P<name>[A-Za-z_]\w*)\s*;", re.DOTALL)
LOWER_SNAKE = re.compile(r"^[a-z][a-z0-9_]*$")


def check(req):
    if req.get("stage") != "cst":
        return []
    payload = req.get("payload") or {}
    ir = payload.get("cst_ir") or {}
    text = payload.get("pp_text") or ir.get("pp_text") or ir.get("source_text") or ""
    line_starts = payload.get("line_starts") or ir.get("line_starts") or [0]
    out = []
    for m in TYPEDEF_ENUM_RE.finditer(text):
        name = m.group("name")
        off = m.start("name")
        loc = byte_span_to_loc(off, off + len(name), line_starts)
        if not name.endswith("_e") or not LOWER_SNAKE.match(name):
            out.append({
                "rule_id": "enum_type_names_lower_snake_e",
                "severity": "warning",
                "message": "enum types should use lower_snake_case and end with _e: {}".format(name),
                "location": loc,
            })
    return out
