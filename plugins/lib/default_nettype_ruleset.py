import re

from lib.dv_helpers import loc, raw_text_inputs

DIRECTIVE_RE = re.compile(r"`default_nettype\s+([A-Za-z_]\w*)", re.IGNORECASE | re.MULTILINE)
PLACEMENT_LIMIT = 20
CACHE_KEY = "__default_nettype_ruleset"


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
    out = []
    matches = list(DIRECTIVE_RE.finditer(text))
    if not matches:
        out.append(_violation("require_default_nettype_head_none", 0, text, "file must declare `default_nettype none` near the top"))
    else:
        first = matches[0]
        value = first.group(1).lower()
        first_loc = loc(text, first.start())
        if value != "none":
            out.append({
                "rule_id": "require_default_nettype_none_value",
                "severity": "warning",
                "message": "`default_nettype` must be set to `none`",
                "location": first_loc,
            })
        else:
            significant = _significant_lines_before(text, first_loc["line"])
            if significant > PLACEMENT_LIMIT:
                out.append({
                    "rule_id": "require_default_nettype_placement",
                    "severity": "warning",
                    "message": f"`default_nettype none` should appear within the first {PLACEMENT_LIMIT} significant lines",
                    "location": first_loc,
                })
        out.extend(_check_trailing_reset(text, matches[-1]))
    table = {}
    for item in out:
        table.setdefault(item["rule_id"], []).append(item)
    req[CACHE_KEY] = table
    return table


def _violation(rule_id, index, text, message):
    return {
        "rule_id": rule_id,
        "severity": "warning",
        "message": message,
        "location": loc(text, index),
    }


def _significant_lines_before(text, target_line):
    count = 0
    in_block = False
    for idx, line in enumerate(text.splitlines(), 1):
        if idx >= target_line:
            break
        stripped = line.strip()
        if not stripped:
            continue
        if in_block:
            if "*/" in stripped:
                in_block = False
            continue
        if stripped.startswith("/*"):
            if "*/" not in stripped:
                in_block = True
            continue
        if stripped.startswith("//"):
            continue
        count += 1
    return count


def _check_trailing_reset(text, last_match):
    out = []
    value = last_match.group(1).lower()
    if value != "wire":
        out.append({
            "rule_id": "require_default_nettype_tail_wire",
            "severity": "warning",
            "message": "`default_nettype none` should be reset to `wire` at the end of the file",
            "location": loc(text, last_match.start()),
        })
    return out
