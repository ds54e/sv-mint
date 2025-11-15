import re

from lib.dv_helpers import loc, raw_text_inputs

WHILE_RE = re.compile(r"\bwhile\s*\(", re.IGNORECASE)


def check(req):
    inputs = raw_text_inputs(req)
    if not inputs:
        return []
    text, _ = inputs
    out = []
    for match in WHILE_RE.finditer(text):
        prefix = text[max(0, match.start() - 20):match.start()]
        if "DV_SPINWAIT" in prefix:
            continue
        out.append({
            "rule_id": "flow.spinwait_macro_required",
            "severity": "warning",
            "message": "wrap while loops in DV_SPINWAIT to add watchdog timers",
            "location": loc(text, match.start()),
        })
    return out
