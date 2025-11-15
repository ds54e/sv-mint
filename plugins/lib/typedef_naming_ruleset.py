import re

from lib.dv_helpers import loc, raw_text_inputs

TYPEDEF_ENUM_RE = re.compile(r"typedef\s+enum(?P<head>[\s\S]*?)\{(?P<body>[\s\S]*?)\}\s*(?P<name>[A-Za-z_]\w*)\s*;", re.DOTALL)
TYPEDEF_RE = re.compile(r"typedef(?!\s+enum).*?\s+([A-Za-z_]\w*)\s*;", re.DOTALL)
LOWER_SNAKE = re.compile(r"^[a-z][a-z0-9_]*$")
CACHE_KEY = "__typedef_naming_ruleset"


def violations_for(req, rule_id):
    table = evaluate(req)
    return list(table.get(rule_id) or [])


def evaluate(req):
    cached = req.get(CACHE_KEY)
    if cached is not None:
        return cached
    inputs = raw_text_inputs(req)
    if not inputs:
        req[CACHE_KEY] = {}
        return req[CACHE_KEY]
    text, _ = inputs
    out = []
    for match in TYPEDEF_ENUM_RE.finditer(text):
        name = match.group("name")
        if not name.endswith("_e"):
            out.append(_violation("typedef.enum_suffix", name, match.start(), text, "enum types should end with _e"))
        if not LOWER_SNAKE.match(name):
            out.append(_violation("typedef.enum_lower_snake", name, match.start("name"), text, "enum types should use lower_snake_case"))
        out.extend(_check_enum_members(text, match))
    for match in TYPEDEF_RE.finditer(text):
        name = match.group(1)
        if not name.endswith("_t"):
            out.append(_violation("typedef.type_suffix", name, match.start(), text, "typedef names should end with _t"))
    table = {}
    for item in out:
        table.setdefault(item["rule_id"], []).append(item)
    req[CACHE_KEY] = table
    return table


def _violation(rule_id, name, index, text, message):
    return {
        "rule_id": rule_id,
        "severity": "warning",
        "message": f"{message}: {name}",
        "location": loc(text, index),
    }


def _check_enum_members(text, match):
    body = match.group("body")
    base = match.group("name")
    prefix = _enum_prefix(base)
    body_start = match.start("body")
    issues = []
    offset = 0
    depth = 0
    token_start = 0
    entries = []
    while offset < len(body):
        ch = body[offset]
        if ch in "([{":
            depth += 1
        elif ch in ")]}":
            depth = max(0, depth - 1)
        elif ch == "," and depth == 0:
            entries.append((body[token_start:offset], token_start))
            token_start = offset + 1
        offset += 1
    entries.append((body[token_start:], token_start))
    for chunk, rel in entries:
        token = chunk.strip()
        if not token:
            continue
        name_match = re.match(r"([A-Za-z_]\w*)", token)
        if not name_match:
            continue
        member = name_match.group(1)
        member_index = body_start + rel + chunk.find(member)
        if not re.match(r"^[A-Z][A-Za-z0-9]*$", member):
            issues.append(_violation("typedef.enum_value_case", member, member_index, text, "enum values should use UpperCamelCase"))
            continue
        if prefix and not member.startswith(prefix):
            issues.append(_violation("typedef.enum_value_prefix", member, member_index, text, f"enum values should start with {prefix}"))
    return issues


def _enum_prefix(name):
    base = name[:-2] if name.endswith("_e") else name
    parts = [p for p in base.split("_") if p]
    if not parts:
        return ""
    return "".join(p[0].upper() + p[1:] for p in parts)
