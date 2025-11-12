import re

TYPEDEF_ENUM_RE = re.compile(r"typedef\s+enum.*?\}\s*([A-Za-z_]\w*)\s*;", re.DOTALL)
TYPEDEF_RE = re.compile(r"typedef(?!\s+enum).*?\s+([A-Za-z_]\w*)\s*;", re.DOTALL)


def check(req):
    if req.get("stage") != "raw_text":
        return []
    payload = req.get("payload") or {}
    text = payload.get("text") or ""
    out = []
    for match in TYPEDEF_ENUM_RE.finditer(text):
        name = match.group(1)
        if not name.endswith("_e"):
            out.append(_violation("typedef.enum_suffix", name, match.start(), text, "enum types should end with _e"))
    for match in TYPEDEF_RE.finditer(text):
        name = match.group(1)
        if not name.endswith("_t"):
            out.append(_violation("typedef.type_suffix", name, match.start(), text, "typedef names should end with _t"))
    return out


def _violation(rule_id, name, index, text, message):
    line = text.count("\n", 0, index) + 1
    prev = text.rfind("\n", 0, index)
    col = index + 1 if prev < 0 else index - prev
    return {
        "rule_id": rule_id,
        "severity": "warning",
        "message": f"{message}: {name}",
        "location": {"line": line, "col": col, "end_line": line, "end_col": col + 1},
    }
