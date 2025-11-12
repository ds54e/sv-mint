import re

COMMA_NO_SPACE = re.compile(r",(?!\s)")
FUNC_SPACE = re.compile(r"\b([A-Za-z_]\w*)\s+\(")
MACRO_SPACE = re.compile(r"`([A-Za-z_]\w*)\s+\(")
CASE_START = re.compile(r"^\s*(unique\s+|priority\s+)?case(x|z)?\b")
CASE_END = re.compile(r"^\s*endcase\b")
RESERVED = {"if", "else", "for", "while", "repeat", "forever", "case", "casex", "casez", "unique", "priority", "return"}


def loc_from_match(text, match):
    start = match.start()
    end = match.end()
    line = text.count("\n", 0, start) + 1
    prev = text.rfind("\n", 0, start)
    col = start + 1 if prev < 0 else start - prev
    end_col = col + (end - start) - 1
    return {"line": line, "col": col, "end_line": line, "end_col": end_col}


def loc_at(text, index, length=1):
    line = text.count("\n", 0, index) + 1
    prev = text.rfind("\n", 0, index)
    col = index + 1 if prev < 0 else index - prev
    return {
        "line": line,
        "col": col,
        "end_line": line,
        "end_col": col + length - 1,
    }


def check(req):
    stage = req.get("stage")
    if stage == "raw_text":
        return comma_and_call_spacing(req)
    if stage == "cst":
        return case_spacing(req)
    return []


def comma_and_call_spacing(req):
    payload = req.get("payload") or {}
    text = payload.get("text") or ""
    out = []
    for match in COMMA_NO_SPACE.finditer(text):
        loc = loc_from_match(text, match)
        out.append({
            "rule_id": "format.comma_space",
            "severity": "warning",
            "message": "missing space after comma",
            "location": loc,
        })
    for match in FUNC_SPACE.finditer(text):
        name = match.group(1)
        if name in RESERVED:
            continue
        if has_identifier_prefix(text, match.start()):
            continue
        loc = loc_from_match(text, match)
        out.append({
            "rule_id": "format.call_spacing",
            "severity": "warning",
            "message": "function or task call must not have space before '('",
            "location": loc,
        })
    for match in MACRO_SPACE.finditer(text):
        loc = loc_from_match(text, match)
        out.append({
            "rule_id": "format.macro_spacing",
            "severity": "warning",
            "message": "macro invocation must not have space before '('",
            "location": loc,
        })
    return out


def case_spacing(req):
    payload = req.get("payload") or {}
    if payload.get("mode") != "inline":
        return []
    ir = payload.get("cst_ir") or {}
    text = ir.get("pp_text") or ""
    line_starts = ir.get("line_starts") or [0]
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
            if colon >= 0:
                global_idx = offset + colon
                before = line[:colon]
                after = line[colon + 1:]
                if before and before[-1] in (" ", "\t"):
                    out.append({
                        "rule_id": "format.case_colon_spacing",
                        "severity": "warning",
                        "message": "case item must not have whitespace before ':'",
                        "location": loc_at(text, global_idx),
                    })
                if not after.startswith(" "):
                    out.append({
                        "rule_id": "format.case_colon_after",
                        "severity": "warning",
                        "message": "case item must have space after ':'",
                        "location": loc_at(text, global_idx),
                    })
        offset += len(line)
    return out


def has_identifier_prefix(text, pos):
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
