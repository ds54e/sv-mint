import re

from lib.raw_text_helpers import byte_loc, raw_inputs

PACKAGE_RE = re.compile(r"(?m)^\s*package\s+([A-Za-z_][\w$]*)")


def check(req):
    inputs = raw_inputs(req)
    if not inputs:
        return []
    text, _ = inputs
    packages = list(PACKAGE_RE.finditer(text))
    if len(packages) <= 1:
        return []
    first = packages[0]
    return [{
        "rule_id": "one_package_per_file",
        "severity": "warning",
        "message": f"multiple package declarations in single file ({first.group(1)})",
        "location": byte_loc(text, first.start()),
    }]
