from lib.dv_helpers import loc

CACHE_KEY = "__format_text_ruleset"


def violations_for(req, rule_id):
    table = evaluate(req)
    return list(table.get(rule_id) or [])


def evaluate(req):
    cached = req.get(CACHE_KEY)
    if cached is not None:
        return cached
    if req.get("stage") != "raw_text":
        req[CACHE_KEY] = {}
        return req[CACHE_KEY]
    payload = req.get("payload") or {}
    text = payload.get("text") or ""
    out = []
    for idx, ch in enumerate(text):
        if ord(ch) > 127:
            out.append({
                "rule_id": "format.ascii_only",
                "severity": "warning",
                "message": "non-ASCII character detected",
                "location": loc(text, idx),
            })
        if ch == "\t":
            out.append({
                "rule_id": "format.no_tabs",
                "severity": "warning",
                "message": "tab character detected",
                "location": loc(text, idx),
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
                    "location": loc(text, j),
                })
                break
            line_start = idx + 1
    if text and not text.endswith("\n"):
        out.append({
            "rule_id": "format.final_newline",
            "severity": "warning",
            "message": "file must end with newline",
            "location": loc(text, len(text) - 1),
        })
    table = {}
    for item in out:
        table.setdefault(item["rule_id"], []).append(item)
    req[CACHE_KEY] = table
    return table
