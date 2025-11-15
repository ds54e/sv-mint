def to_loc(text, index, length=4):
    line = text.count("\n", 0, index) + 1
    prev_newline = text.rfind("\n", 0, index)
    if prev_newline < 0:
        col = index + 1
    else:
        col = index - prev_newline
    return {
        "line": line,
        "col": col,
        "end_line": line,
        "end_col": col + length - 1,
    }


def check(req):
    if req.get("stage") != "pp_text":
        return []
    payload = req.get("payload") or {}
    text = payload.get("text") or ""
    out = []
    idx = 0
    while True:
        pos = text.find("end", idx)
        if pos < 0:
            break
        if pos > 0 and text[pos - 1].isalnum():
            idx = pos + 1
            continue
        after = pos + 3
        while after < len(text) and text[after] in " \t":
            after += 1
        if after < len(text) and text[after] == '\n':
            after += 1
            while after < len(text) and text[after] in " \t":
                after += 1
            if text.startswith("else", after):
                loc = to_loc(text, after, length=4)
                out.append({
                    "rule_id": "format.end_else_inline",
                    "severity": "warning",
                    "message": "else must be on the same line as the preceding end",
                    "location": loc,
                })
        idx = pos + 1
    return out
