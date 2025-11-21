import re

from lib.dv_helpers import loc, raw_text_inputs

DIRECTIVE_RE = re.compile(r"`default_nettype\s+([A-Za-z_]\w*)", re.IGNORECASE | re.MULTILINE)


def check(req):
    if req.get("stage") != "raw_text":
        return []
    inputs = raw_text_inputs(req)
    if not inputs:
        return []
    text, _ = inputs
    matches = list(DIRECTIVE_RE.finditer(text))
    if not matches:
        return []
    last = matches[-1]
    value = last.group(1).lower()
    if value == "wire":
        return []
    return [{
        "rule_id": "default_nettype_ends_with_wire",
        "severity": "warning",
        "message": "`default_nettype none` should be reset to `wire` at the end of the file",
        "location": loc(text, last.start()),
    }]
