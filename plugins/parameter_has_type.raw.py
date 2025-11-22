import re

from lib.raw_text_helpers import byte_loc, raw_inputs

PARAM_RE = re.compile(r"\bparameter\b", re.IGNORECASE)
TOKEN_RE = re.compile(r"[A-Za-z_]\w*|\[")
TYPED_TOKENS = {
    "type",
    "bit",
    "logic",
    "reg",
    "int",
    "integer",
    "longint",
    "shortint",
    "byte",
    "time",
    "realtime",
    "real",
    "shortreal",
    "string",
    "wire",
    "signed",
    "unsigned",
}


def check(req):
    inputs = raw_inputs(req)
    if not inputs:
        return []
    text, _ = inputs
    out = []
    for m in PARAM_RE.finditer(text):
        token = _first_token(text, m.end())
        if token is None:
            continue
        if token == "[" or token.lower() in TYPED_TOKENS:
            continue
        loc = byte_loc(text, m.start())
        out.append({
            "rule_id": "parameter_has_type",
            "severity": "warning",
            "message": "parameter must declare an explicit data type",
            "location": loc,
        })
    return out


def _first_token(text, start):
    m = TOKEN_RE.search(text, pos=start)
    if not m:
        return None
    return m.group(0)
