import re

from lib.raw_text_helpers import byte_loc, raw_inputs

DIRECTIVE_RE = re.compile(r"`default_nettype\s+([A-Za-z_]\w*)", re.IGNORECASE | re.MULTILINE)


def check(req):
    inputs = raw_inputs(req)
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
        "location": byte_loc(text, 0),
    }]
