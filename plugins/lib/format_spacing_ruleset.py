import re

from lib.dv_helpers import loc

CACHE_KEY = "__format_spacing_ruleset"
COMMA_NO_SPACE = re.compile(r",(?!\s)")
FUNC_SPACE = re.compile(r"\b([A-Za-z_]\w*)\s+\(")
MACRO_SPACE = re.compile(r"`([A-Za-z_]\w*)\s+\(")
CASE_START = re.compile(r"^\s*(unique\s+|priority\s+)?case(x|z)?\b")
CASE_END = re.compile(r"^\s*endcase\b")
RESERVED = {"if", "else", "for", "while", "repeat", "forever", "case", "casex", "casez", "unique", "priority", "return"}


def violations_for(req, rule_id):
    table = evaluate(req)
    return list(table.get(rule_id) or [])


def evaluate(req):
    cached = req.get(CACHE_KEY)
    if cached is not None:
        return cached
    stage = req.get("stage")
    if stage == "raw_text":
        table = _raw_spacing(req)
    elif stage == "cst":
        table = _case_spacing(req)
    else:
        table = {}
    req[CACHE_KEY] = table
    return table


def _raw_spacing(req):
    payload = req.get("payload") or {}
    text = payload.get("text") or ""
    items = []
    for match in COMMA_NO_SPACE.finditer(text):
        items.append({
            "rule_id": "format.comma_space",
            "severity": "warning",
            "message": "missing space after comma",
            "location": _loc_from_match(text, match),
        })
    for match in FUNC_SPACE.finditer(text):
        name = match.group(1)
        if name in RESERVED:
            continue
        if _has_identifier_prefix(text, match.start()):
            continue
        items.append({
            "rule_id": "format.call_spacing",
            "severity": "warning",
            "message": "function or task call must not have space before '('",
            "location": _loc_from_match(text, match),
        })
    for match in MACRO_SPACE.finditer(text):
        items.append({
            "rule_id": "format.macro_spacing",
            "severity": "warning",
            "message": "macro invocation must not have space before '('",
            "location": _loc_from_match(text, match),
        })
    return _group_by_rule(items)


def _case_spacing(req):
    payload = req.get("payload") or {}
    if payload.get("mode") != "inline":
        return {}
    ir = payload.get("cst_ir") or {}
    text = ir.get("pp_text") or ""
    out = []
    offset = 0
    depth = 0
    for line in text.splitlines(keepends=True):
        trimmed = line.strip()
        if CASE_START.match(trimmed):
            depth += 1
        elif CASE_END.match(trimmed):
            depth = max(depth - 1, 0)
        elif depth > 0 and ":" in line:
            colon = line.find(":")
            global_idx = offset + colon
            before = line[:colon]
            after = line[colon + 1:]
            if before and before[-1] in (" ", "\t"):
                out.append({
                    "rule_id": "format.case_colon_spacing",
                    "severity": "warning",
                    "message": "case item must not have whitespace before ':'",
                    "location": loc(text, global_idx),
                })
            if not after.startswith(" "):
                out.append({
                    "rule_id": "format.case_colon_after",
                    "severity": "warning",
                    "message": "case item must have space after ':'",
                    "location": loc(text, global_idx),
                })
        offset += len(line)
    return _group_by_rule(out)


def _group_by_rule(items):
    table = {}
    for item in items:
        table.setdefault(item["rule_id"], []).append(item)
    return table


def _loc_from_match(text, match):
    start = match.start()
    end = match.end()
    line = text.count("\n", 0, start) + 1
    prev = text.rfind("\n", 0, start)
    col = start + 1 if prev < 0 else start - prev
    end_col = col + (end - start) - 1
    return {"line": line, "col": col, "end_line": line, "end_col": end_col}


def _has_identifier_prefix(text, pos):
    line_start = text.rfind("\n", 0, pos)
    if line_start < 0:
        line_start = 0
    else:
        line_start += 1
    prefix = text[line_start:pos]
    stripped = prefix.rstrip()
    if not stripped:
        return False
    if "function" in stripped.split() or "task" in stripped.split():
        return True
    last_char = stripped[-1]
    return last_char.isalpha() or last_char == "_"
