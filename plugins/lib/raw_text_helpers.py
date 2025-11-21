"""Raw text stage helpers used by multiple plugins."""


def raw_inputs(req):
    if req.get("stage") != "raw_text":
        return None
    payload = req.get("payload") or {}
    text = payload.get("text") or ""
    path = req.get("path") or ""
    return text, path


def byte_loc(text, index):
    line = text.count("\n", 0, index) + 1
    prev = text.rfind("\n", 0, index)
    col = index + 1 if prev < 0 else index - prev
    return {
        "line": line,
        "col": col,
        "end_line": line,
        "end_col": col + 1,
    }
