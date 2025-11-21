import re

from lib.dv_helpers import loc, raw_text_inputs

DEFINE_PATTERN = re.compile(r"(?m)^\s*`define\s+([A-Za-z_]\w*)")
USE_PATTERN = re.compile(r"`([A-Za-z_]\w*)")
CACHE_KEY = "__macro_usage_rules"


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
    defines = []
    for m in DEFINE_PATTERN.finditer(text):
        defines.append((m.group(1), m.start()))
    uses = {m.group(1) for m in USE_PATTERN.finditer(text)}
    out = []
    for name, start in defines:
        if name in uses:
            continue
        if _has_unused_comment(text, start):
            continue
        out.append({
            "rule_id": "macro.no_unused_macro",
            "severity": "warning",
            "message": f"macro `{name}` is defined but never used",
            "location": loc(text, start),
        })
    table = {}
    for item in out:
        table.setdefault(item["rule_id"], []).append(item)
    req[CACHE_KEY] = table
    return table


def _has_unused_comment(text, offset):
    line_start = text.rfind("\n", 0, offset)
    if line_start == -1:
        line_start = 0
    else:
        line_start += 1
    line_end = text.find("\n", offset)
    if line_end == -1:
        line_end = len(text)
    segment = text[line_start:line_end]
    idx = offset - line_start
    if idx < 0:
        idx = 0
    rest = segment[idx:]
    pos = rest.find("//")
    if pos != -1 and "unused" in rest[pos + 2 :].lower():
        return True
    pos_block = rest.find("/*")
    if pos_block != -1:
        end = rest.find("*/", pos_block + 2)
        if end != -1 and "unused" in rest[pos_block + 2 : end].lower():
            return True
    return False
