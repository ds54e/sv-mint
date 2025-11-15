KEYWORDS_PAREN = ("if", "for", "foreach", "while", "repeat")
KEYWORDS_NOPAREN = ("forever",)


def locate(text, index):
    line = text.count("\n", 0, index) + 1
    prev = text.rfind("\n", 0, index)
    col = index + 1 if prev < 0 else index - prev
    return {"line": line, "col": col, "end_line": line, "end_col": col + 1}


def is_word_boundary(ch):
    return ch is None or not (ch.isalnum() or ch == "_")


def match_paren(text, start):
    depth = 0
    i = start
    while i < len(text):
        ch = text[i]
        if ch == "(":
            depth += 1
        elif ch == ")":
            depth -= 1
            if depth == 0:
                return i
        i += 1
    return -1


def skip_whitespace(text, index):
    newline = False
    i = index
    while i < len(text):
        ch = text[i]
        if ch in " \t":
            i += 1
            continue
        if ch in "\r\n":
            newline = True
            i += 1
            continue
        break
    return i, newline


def check(req):
    if req.get("stage") != "pp_text":
        return []
    payload = req.get("payload") or {}
    text = payload.get("text") or ""
    if not text:
        return []
    lower = text.lower()
    out = []
    i = 0
    while i < len(lower):
        found = None
        keyword = None
        for kw in KEYWORDS_PAREN + KEYWORDS_NOPAREN:
            if lower.startswith(kw, i):
                found = i
                keyword = kw
                break
        if found is None:
            i += 1
            continue
        before = lower[found - 1] if found > 0 else None
        if not is_word_boundary(before):
            i += 1
            continue
        after = found + len(keyword)
        if after < len(lower) and not is_word_boundary(lower[after]):
            i += 1
            continue
        if keyword in KEYWORDS_PAREN:
            if keyword == "if" and found > 0 and lower[found - 4:found] == "else":
                pass
            after, _ = skip_whitespace(lower, after)
            if after >= len(lower) or lower[after] != "(":
                i = after + 1
                continue
            close = match_paren(lower, after)
            if close < 0:
                i = after + 1
                continue
            stmt_start, newline = skip_whitespace(lower, close + 1)
        else:
            stmt_start, newline = skip_whitespace(lower, after)
        if newline:
            if not lower.startswith("begin", stmt_start):
                loc = locate(text, stmt_start)
                out.append({
                    "rule_id": "format.begin_required",
                    "severity": "warning",
                    "message": f"{keyword} body must start with begin when split across lines",
                    "location": loc,
                })
        i = found + 1
    return out
