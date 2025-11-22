import re
from pathlib import Path

from lib.utf8 import line_starts, span_to_loc

def check(req):
    if req.get("stage") != "cst":
        return []
    payload = req.get("payload") or {}
    ir = payload.get("cst_ir") or {}
    text = payload.get("pp_text") or ir.get("pp_text") or ir.get("source_text") or ""
    starts = payload.get("line_starts") or ir.get("line_starts")
    if not text:
        path = req.get("path")
        if path:
            try:
                text = Path(path).read_text(encoding="utf-8")
            except Exception:
                text = ""
    if not starts:
        starts = line_starts(text) if text else [0]
    matches = list(DEFAULT_RE.finditer(text))
    if not matches:
        return []
    last = matches[-1]
    value = last.group("value").lower()
    if value == "wire":
        return []
    loc = span_to_loc(text, last.start(), last.end(), starts)
    return [
        {
            "rule_id": "default_nettype_ends_with_wire",
            "severity": "warning",
            "message": "`default_nettype none` should be reset to `wire` at the end of the file",
            "location": loc,
        }
    ]


DEFAULT_RE = re.compile(r"`\s*default_nettype\s+(?P<value>\w+)", re.IGNORECASE)
