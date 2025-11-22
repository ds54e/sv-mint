import re

from lib.cst_inline import byte_span_to_loc

TYPEDEF_RE = re.compile(
    r"typedef(?!\s+enum).*?\s+(?P<name>[A-Za-z_]\w*)\s*;", re.DOTALL
)
LOWER_SNAKE = re.compile(r"^[a-z][a-z0-9_]*$")

def check(req):
    if req.get("stage") != "cst":
        return []
    payload = req.get("payload") or {}
    ir = payload.get("cst_ir") or {}
    text = payload.get("pp_text") or ir.get("pp_text") or ir.get("source_text") or ""
    line_starts = payload.get("line_starts") or ir.get("line_starts") or [0]
    out = []
    for m in TYPEDEF_RE.finditer(text):
        name = m.group("name")
        if name.endswith("_e"):
            continue
        off = m.start("name")
        loc = byte_span_to_loc(off, off + len(name), line_starts)
        if not name.endswith("_t") or not LOWER_SNAKE.match(name):
            out.append(
                {
                    "rule_id": "typedef_names_lower_snake_t",
                    "severity": "warning",
                    "message": f"typedef names should use lower_snake_case and end with _t: {name}",
                    "location": loc,
                }
            )
    return out
