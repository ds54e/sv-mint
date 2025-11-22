import re

from lib.cst_inline import byte_span_to_loc

TYPEDEF_ENUM_RE = re.compile(
    r"typedef\s+enum(?P<head>[\s\S]*?)\{(?P<body>[\s\S]*?)\}\s*(?P<name>[A-Za-z_]\w*)\s*;",
    re.DOTALL,
)
UPPER_CAMEL = re.compile(r"^[A-Z][A-Za-z0-9]*$")
ALL_CAPS = re.compile(r"^[A-Z][A-Z0-9_]*$")

def check(req):
    if req.get("stage") != "cst":
        return []
    payload = req.get("payload") or {}
    ir = payload.get("cst_ir") or {}
    text = payload.get("pp_text") or ir.get("pp_text") or ir.get("source_text") or ""
    line_starts = payload.get("line_starts") or ir.get("line_starts") or [0]
    out = []
    for m in TYPEDEF_ENUM_RE.finditer(text):
        body = m.group("body")
        body_off = m.start("body")
        for member, rel in _enum_entries(body):
            off = body_off + rel
            loc = byte_span_to_loc(off, off + len(member), line_starts)
            if not (UPPER_CAMEL.match(member) or ALL_CAPS.match(member)):
                out.append(
                    {
                        "rule_id": "enum_values_uppercase",
                        "severity": "warning",
                        "message": "enum values should use UpperCamelCase or ALL_CAPS: {}".format(
                            member
                        ),
                        "location": loc,
                    }
                )
    return out

def _enum_entries(body):
    out = []
    depth = 0
    token_start = 0
    for idx, ch in enumerate(body):
        if ch in "([{":
            depth += 1
        elif ch in ")]}":
            depth = max(0, depth - 1)
        elif ch == "," and depth == 0:
            name = _head_ident(body[token_start:idx])
            if name:
                rel = token_start + body[token_start:idx].find(name)
                out.append((name, rel))
            token_start = idx + 1
    name = _head_ident(body[token_start:])
    if name:
        rel = token_start + body[token_start:].find(name)
        out.append((name, rel))
    return out

def _head_ident(chunk):
    m = re.match(r"\s*([A-Za-z_]\w*)", chunk)
    return m.group(1) if m else None
