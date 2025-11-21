import re

from lib.dv_helpers import loc, raw_text_inputs

PACKAGE_RE = re.compile(r"(?m)^\s*package\s+([A-Za-z_][\w$]*)")
ENDPACKAGE_RE = re.compile(r"(?m)^\s*endpackage(?:\s*:\s*([A-Za-z_][\w$]*))?")
DEFINE_RE = re.compile(r"(?m)^\s*`define\s+([A-Za-z_][\w$]*)")
CACHE_KEY = "__package_ruleset"


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
    packages = list(PACKAGE_RE.finditer(text))
    endpackages = list(ENDPACKAGE_RE.finditer(text))
    if len(packages) > 1:
        match = packages[0]
        out.append(_violation(
            "package_single_package",
            f"multiple package declarations in single file ({match.group(1)})",
            match.start(),
            text,
        ))
    if packages:
        expected = packages[0].group(1)
        if not endpackages:
            out.append(_violation(
                "package.require_endpackage",
                f"package {expected} missing endpackage",
                packages[0].start(),
                text,
            ))
        else:
            label = endpackages[-1].group(1)
            if label and label != expected:
                pos = endpackages[-1].start()
                out.append(_violation(
                    "package.endpackage_label_match",
                    f"endpackage label {label} does not match package {expected}",
                    pos,
                    text,
                ))
        if endpackages:
            body_start = packages[0].end()
            body_end = endpackages[0].start()
            body = text[body_start:body_end]
            for match in DEFINE_RE.finditer(body):
                name = match.group(1)
                out.append(_violation(
                    "package_no_define_in_package",
                    f"prefer parameters over `define {name} inside package",
                    body_start + match.start(),
                    text,
                ))
    table = {}
    for item in out:
        table.setdefault(item["rule_id"], []).append(item)
    req[CACHE_KEY] = table
    return table


def _violation(rule_id, message, index, text):
    return {
        "rule_id": rule_id,
        "severity": "warning",
        "message": message,
        "location": loc(text, index),
    }
