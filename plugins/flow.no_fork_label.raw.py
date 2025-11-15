import re

from lib.dv_helpers import loc, raw_text_inputs

FORK_LABEL_RE = re.compile(r"\bfork\s*:", re.IGNORECASE)


def check(req):
    inputs = raw_text_inputs(req)
    if not inputs:
        return []
    text, _ = inputs
    out = []
    for match in FORK_LABEL_RE.finditer(text):
        out.append({
            "rule_id": "flow.no_fork_label",
            "severity": "warning",
            "message": "avoid fork labels; use DV_SPINWAIT isolation blocks instead",
            "location": loc(text, match.start()),
        })
    return out
