def to_loc(text, index, length=1):
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
    if req.get("stage") != "raw_text":
        return []
    payload = req.get("payload") or {}
    text = payload.get("text") or ""
    out = []
    for idx, ch in enumerate(text):
        if ord(ch) > 127:
            out.append({
                "rule_id": "format.ascii_only",
                "severity": "warning",
                "message": "non-ASCII character detected",
                "location": to_loc(text, idx),
            })
        if ch == "\t":
            out.append({
                "rule_id": "format.no_tabs",
                "severity": "warning",
                "message": "tab character detected",
                "location": to_loc(text, idx),
            })
    line_start = 0
    for idx, ch in enumerate(text):
        if ch == "\n":
            j = idx - 1
            while j >= line_start and text[j] in (" ", "\t"):
                out.append({
                    "rule_id": "format.no_trailing_whitespace",
                    "severity": "warning",
                    "message": "trailing whitespace at line end",
                    "location": to_loc(text, j),
                })
                break
            line_start = idx + 1
    if text and not text.endswith("\n"):
        out.append({
            "rule_id": "format.final_newline",
            "severity": "warning",
            "message": "file must end with newline",
            "location": to_loc(text, len(text) - 1),
        })
    return out
