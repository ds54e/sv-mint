import re

from lib.dv_helpers import loc, raw_text_inputs

WAIT_FORK_RE = re.compile(r"\bwait\s+fork\b", re.IGNORECASE)


def check(req):
    inputs = raw_text_inputs(req)
    if not inputs:
        return []
    text, _ = inputs
    out = []
    for match in WAIT_FORK_RE.finditer(text):
        out.append({
            "rule_id": "flow.wait_fork_isolation",
            "severity": "warning",
            "message": "wait fork must be wrapped in an isolation fork (prefer DV_SPINWAIT)",
            "location": loc(text, match.start()),
        })
    return out
