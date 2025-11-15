MARK = "__SV_MINT_TEMPLATE__"


def locate(text, index, length):
    line = text.count("\n", 0, index) + 1
    last_newline = text.rfind("\n", 0, index)
    if last_newline < 0:
        col = index + 1
    else:
        col = index - last_newline
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
    index = text.find(MARK)
    if index < 0:
        return []
    loc = locate(text, index, len(MARK))
    return [
        {
            "rule_id": "template.raw_text_marker",
            "severity": "info",
            "message": "template marker detected",
            "location": loc,
        }
    ]
