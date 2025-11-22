import re

PACKAGE_RE = re.compile(r"(?m)^\s*package\s+([A-Za-z_][\w$]*)")

def check(req):
    if req.get("stage") != "raw_text":
        return []
    payload = req.get("payload") or {}
    text = payload.get("text") or ""
    packages = list(PACKAGE_RE.finditer(text))
    if len(packages) <= 1:
        return []
    first = packages[0]
    return [
        {
            "rule_id": "one_package_per_file",
            "severity": "warning",
            "message": f"multiple package declarations in single file ({first.group(1)})",
            "location": _byte_loc(text, first.start()),
        }
    ]

def _byte_loc(text, index):
    line = text.count("\n", 0, index) + 1
    prev = text.rfind("\n", 0, index)
    col = index + 1 if prev < 0 else index - prev
    return {"line": line, "col": col, "end_line": line, "end_col": col + 1}
