import re

PACKAGE_RE = re.compile(r"(?m)^\s*package\s+([A-Za-z_][\w$]*)")
ENDPACKAGE_RE = re.compile(r"(?m)^\s*endpackage(?:\s*:\s*([A-Za-z_][\w$]*))?")
DEFINE_RE = re.compile(r"(?m)^\s*`define\s+([A-Za-z_][\w$]*)")


def check(req):
    if req.get("stage") != "raw_text":
        return []
    payload = req.get("payload") or {}
    text = payload.get("text") or ""
    out = []
    packages = PACKAGE_RE.findall(text)
    endpackages = ENDPACKAGE_RE.findall(text)
    if len(packages) > 1:
        for match in PACKAGE_RE.finditer(text):
            out.append(_violation(
                "package.multiple",
                f"multiple package declarations in single file ({match.group(1)})",
                match.start(),
                text,
            ))
            break
    if packages:
        expected = packages[0]
        if not endpackages:
            out.append(_violation(
                "package.missing_end",
                f"package {expected} missing endpackage",
                text.rfind(packages[0]),
                text,
            ))
        else:
            label = endpackages[-1]
            if label and label != expected:
                pos = ENDPACKAGE_RE.search(text).start()
                out.append(_violation(
                    "package.end_mismatch",
                    f"endpackage label {label} does not match package {expected}",
                    pos,
                    text,
                ))
        if packages and endpackages:
            start = PACKAGE_RE.search(text).end()
            end = ENDPACKAGE_RE.search(text).start()
            body = text[start:end]
            for match in DEFINE_RE.finditer(body):
                name = match.group(1)
                if not name.startswith("_"):
                    out.append(_violation(
                        "package.define_in_package",
                        f"prefer parameters over `define {name} inside package",
                        match.start(),
                        text,
                    ))
    return out


def _violation(rule_id, message, index, text):
    line = text.count("\n", 0, index) + 1
    prev = text.rfind("\n", 0, index)
    col = index + 1 if prev < 0 else index - prev
    return {
        "rule_id": rule_id,
        "severity": "warning",
        "message": message,
        "location": {
            "line": line,
            "col": col,
            "end_line": line,
            "end_col": col + 1,
        },
    }
