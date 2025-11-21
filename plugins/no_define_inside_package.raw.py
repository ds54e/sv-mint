import re

from lib.dv_helpers import loc, raw_text_inputs

PACKAGE_RE = re.compile(r"(?m)^\s*package\s+([A-Za-z_][\w$]*)")
ENDPACKAGE_RE = re.compile(r"(?m)^\s*endpackage(?:\s*:\s*([A-Za-z_][\w$]*))?")
DEFINE_RE = re.compile(r"(?m)^\s*`define\s+([A-Za-z_][\w$]*)")


def check(req):
    if req.get("stage") != "raw_text":
        return []
    inputs = raw_text_inputs(req)
    if not inputs:
        return []
    text, _ = inputs
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
        out.append({
            "rule_id": "no_define_inside_package",
            "severity": "warning",
            "message": f"prefer parameters over `define {name} inside package",
            "location": loc(text, body_start + match.start()),
        })
    return out
