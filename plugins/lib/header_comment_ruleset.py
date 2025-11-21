import re

from lib.dv_helpers import raw_text_inputs

SPDX_RE = re.compile(r"SPDX-License-Identifier")
CACHE_KEY = "__header_comment_ruleset"


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
    snippet = text[:200]
    issues = []
    location = {"line": 1, "col": 1, "end_line": 1, "end_col": 1}
    if not SPDX_RE.search(snippet):
        issues.append({
            "rule_id": "require_header_spdx",
            "severity": "warning",
            "message": "file should include SPDX-License-Identifier header",
            "location": location,
        })
    lines = text.splitlines()
    if lines:
        has_comment = any(line.strip().startswith("//") for line in lines[:5])
        if not has_comment:
            issues.append({
                "rule_id": "require_header_comment",
                "severity": "warning",
                "message": "file header should include descriptive comment",
                "location": location,
            })
    table = {}
    for item in issues:
        table.setdefault(item["rule_id"], []).append(item)
    req[CACHE_KEY] = table
    return table
