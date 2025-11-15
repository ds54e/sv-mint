import re

from lib.dv_helpers import loc, raw_text_inputs

DEFINE_PATTERN = re.compile(r"(?m)^\s*`define\s+([A-Za-z_]\w*)")
UNDEF_PATTERN = re.compile(r"(?m)^\s*`undef\s+([A-Za-z_]\w*)")
CACHE_KEY = "__global_define_ruleset"


def violations_for(req, rule_id):
    table = evaluate(req)
    return list(table.get(rule_id) or [])


def evaluate(req):
    cached = req.get(CACHE_KEY)
    if cached is not None:
        return cached
    inputs = raw_text_inputs(req)
    if not inputs:
        req[CACHE_KEY] = {}
        return req[CACHE_KEY]
    text, _ = inputs
    undef_names = {m.group(1) for m in UNDEF_PATTERN.finditer(text)}
    out = []
    for match in DEFINE_PATTERN.finditer(text):
        name = match.group(1)
        location = loc(text, match.start())
        if name.startswith("_"):
            if name not in undef_names:
                out.append({
                    "rule_id": "global.local_define_undef",
                    "severity": "warning",
                    "message": f"local macro {name} must be undefined after use",
                    "location": location,
                })
        else:
            out.append({
                "rule_id": "global.prefer_parameters",
                "severity": "warning",
                "message": f"use parameters instead of global macro `{name}`",
                "location": location,
            })
    table = {}
    for item in out:
        table.setdefault(item["rule_id"], []).append(item)
    req[CACHE_KEY] = table
    return table
