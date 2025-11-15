MAX_COLUMNS = 100


def check(req):
    if req.get("stage") != "raw_text":
        return []
    payload = req.get("payload") or {}
    text = payload.get("text") or ""
    if not text:
        return []
    violations = []
    for idx, line in enumerate(text.splitlines()):
        length = len(line)
        if length > MAX_COLUMNS:
            location = {
                "line": idx + 1,
                "col": MAX_COLUMNS + 1,
                "end_line": idx + 1,
                "end_col": length + 1,
            }
            violations.append({
                "rule_id": "format.line_length",
                "severity": "warning",
                "message": f"line exceeds {MAX_COLUMNS} columns ({length})",
                "location": location,
            })
    return violations
