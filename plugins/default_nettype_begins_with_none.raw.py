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
    if matches:
        # Existing behavior only reports when the directive is missing entirely.
        return []
    return [{
        "rule_id": "default_nettype_begins_with_none",
        "severity": "warning",
        "message": "file must declare `default_nettype none` near the top",
        "location": loc(text, 0),
    }]
