import re

PACKAGE_RE = re.compile(r"(?m)^\s*package\s+([A-Za-z_][\w$]*)")
ENDPACKAGE_RE = re.compile(r"(?m)^\s*endpackage(?:\s*:\s*([A-Za-z_][\w$]*))?")
DEFINE_RE = re.compile(r"(?m)^\s*`define\s+([A-Za-z_][\w$]*)")

def check(req):
    if req.get("stage") != "raw_text":
        return []
    payload = req.get("payload") or {}
    text = payload.get("text") or ""
    packages = list(PACKAGE_RE.finditer(text))
    endpackages = list(ENDPACKAGE_RE.finditer(text))
    if not packages or not endpackages:
        return []
    body_start = packages[0].end()
    body_end = endpackages[0].start()
    body = text[body_start:body_end]
    out = []
    for match in DEFINE_RE.finditer(body):
        name = match.group(1)
        out.append(
            {
                "rule_id": "no_define_inside_package",
                "severity": "warning",
                "message": f"prefer parameters over `define {name} inside package",
                "location": _byte_loc(text, body_start + match.start()),
            }
        )
    return out

def _byte_loc(text, index):
    line = text.count("\n", 0, index) + 1
    prev = text.rfind("\n", 0, index)
    col = index + 1 if prev < 0 else index - prev
    return {"line": line, "col": col, "end_line": line, "end_col": col + 1}
