import re

from lib.dv_helpers import loc, raw_text_inputs

WAIT_STMT_RE = re.compile(r"\bwait\s*\(", re.IGNORECASE)


def check(req):
    inputs = raw_text_inputs(req)
    if not inputs:
        return []
    text, _ = inputs
    out = []
    for match in WAIT_STMT_RE.finditer(text):
        out.append({
            "rule_id": "flow.wait_macro_required",
            "severity": "warning",
            "message": "use DV_WAIT macro instead of raw wait statements",
            "location": loc(text, match.start()),
        })
    return out
