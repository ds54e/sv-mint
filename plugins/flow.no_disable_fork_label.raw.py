import re

from lib.dv_helpers import loc, raw_text_inputs

DISABLE_FORK_LABEL_RE = re.compile(r"\bdisable\s+[A-Za-z_]\w*\s*;", re.IGNORECASE)


def check(req):
    inputs = raw_text_inputs(req)
    if not inputs:
        return []
    text, _ = inputs
    out = []
    for match in DISABLE_FORK_LABEL_RE.finditer(text):
        out.append({
            "rule_id": "flow.no_disable_fork_label",
            "severity": "warning",
            "message": "use disable fork/thread inside isolation fork instead of disable fork_label",
            "location": loc(text, match.start()),
        })
    return out
