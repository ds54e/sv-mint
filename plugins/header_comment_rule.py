import re

SPDX_RE = re.compile(r"SPDX-License-Identifier")


def check(req):
    if req.get("stage") != "raw_text":
        return []
    payload = req.get("payload") or {}
    text = payload.get("text") or ""
    snippet = text[:200]
    if SPDX_RE.search(snippet):
        return []
    return [{
        "rule_id": "header.missing_spdx",
        "severity": "warning",
        "message": "file should include SPDX-License-Identifier header",
        "location": {"line": 1, "col": 1, "end_line": 1, "end_col": 1},
    }]
