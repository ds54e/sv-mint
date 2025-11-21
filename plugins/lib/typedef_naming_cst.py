import re

from lib.cst_inline import byte_span_to_loc

TYPEDEF_ENUM_RE = re.compile(r"typedef\s+enum(?P<head>[\s\S]*?)\{(?P<body>[\s\S]*?)\}\s*(?P<name>[A-Za-z_]\w*)\s*;", re.DOTALL)
TYPEDEF_RE = re.compile(r"typedef(?!\s+enum).*?\s+(?P<name>[A-Za-z_]\w*)\s*;", re.DOTALL)
LOWER_SNAKE = re.compile(r"^[a-z][a-z0-9_]*$")
UPPER_CAMEL = re.compile(r"^[A-Z][A-Za-z0-9]*$")
CACHE_KEY = "__typedef_naming_cst"


def violations_for(req, rule_id):
    table = evaluate(req)
    return list(table.get(rule_id) or [])


def evaluate(req):
    cached = req.get(CACHE_KEY)
    if cached is not None:
        return cached
    if req.get("stage") != "cst":
        req[CACHE_KEY] = {}
        return req[CACHE_KEY]
    payload = req.get("payload") or {}
    ir = payload.get("cst_ir") or {}
    text = payload.get("pp_text") or ir.get("pp_text") or ""
    line_starts = payload.get("line_starts") or ir.get("line_starts") or [0]
    out = []
    out.extend(_check_enum(text, 0, line_starts))
    out.extend(_check_typedef(text, 0, line_starts))
    table = {}
    for item in out:
        table.setdefault(item["rule_id"], []).append(item)
    req[CACHE_KEY] = table
    return table


def _check_enum(snippet, base, line_starts):
    out = []
    for m in TYPEDEF_ENUM_RE.finditer(snippet):
        name = m.group("name")
        name_off = base + m.start("name")
        name_loc = byte_span_to_loc(name_off, name_off + len(name), line_starts)
        if not name.endswith("_e"):
            out.append(_make("typedef.enum_suffix", f"enum types should end with _e: {name}", name_loc))
        if not LOWER_SNAKE.match(name):
            out.append(_make("typedef.enum_lower_snake", f"enum types should use lower_snake_case: {name}", name_loc))
        body = m.group("body")
        body_off = base + m.start("body")
        prefix = _enum_prefix(name)
        for member, rel in _enum_entries(body):
            off = body_off + rel
            loc = byte_span_to_loc(off, off + len(member), line_starts)
            if not UPPER_CAMEL.match(member):
                out.append(_make("typedef.enum_value_case", f"enum values should use UpperCamelCase: {member}", loc))
    return out


def _check_typedef(snippet, base, line_starts):
    out = []
    for m in TYPEDEF_RE.finditer(snippet):
        name = m.group("name")
        if name.endswith("_e"):
            continue
        name_off = base + m.start("name")
        name_loc = byte_span_to_loc(name_off, name_off + len(name), line_starts)
        if not name.endswith("_t"):
            out.append(_make("typedef.type_suffix", f"typedef names should end with _t: {name}", name_loc))
    return out


def _enum_entries(body):
    entries = []
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
                entries.append((name, rel))
            token_start = idx + 1
    name = _head_ident(body[token_start:])
    if name:
        rel = token_start + body[token_start:].find(name)
        entries.append((name, rel))
    return entries


def _head_ident(chunk):
    m = re.match(r"\s*([A-Za-z_]\w*)", chunk)
    return m.group(1) if m else None


def _enum_prefix(name):
    base = name[:-2] if name.endswith("_e") else name
    parts = [p for p in base.split("_") if p]
    return "".join(p.capitalize() for p in parts)


def _make(rule_id, message, loc):
    return {
        "rule_id": rule_id,
        "severity": "warning",
        "message": message,
        "location": loc,
    }
