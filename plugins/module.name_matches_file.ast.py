import re
from pathlib import Path


def check(req):
    if req.get("stage") != "ast":
        return []
    path = req.get("path") or ""
    stem = Path(path).stem if path else None
    payload = req.get("payload") or {}
    decls = payload.get("decls") or []
    out = []
    if stem:
        for d in decls:
            if d.get("kind") != "module":
                continue
            name = d.get("name")
            if name and name != stem:
                out.append({
                    "rule_id": "module.name_matches_file",
                    "severity": "warning",
                    "message": f"module name {name} should match file name {stem}",
                    "location": d.get("loc"),
                })
        out.extend(_check_package_name(path, stem))
    return out


def _check_package_name(path, stem):
    if not path or not stem:
        return []
    try:
        text = Path(path).read_text(encoding="utf-8")
    except OSError:
        return []
    m = re.search(r"\bpackage\s+([A-Za-z_]\w*)", text)
    if not m:
        return []
    name = m.group(1)
    if name == stem:
        return []
    start = m.start(1)
    loc = _loc(text, start)
    return [{
        "rule_id": "module.name_matches_file",
        "severity": "warning",
        "message": f"package name {name} should match file name {stem}",
        "location": loc,
    }]


def _loc(text, index):
    line = text.count("\n", 0, index) + 1
    prev = text.rfind("\n", 0, index)
    col = index + 1 if prev < 0 else index - prev
    return {
        "line": line,
        "col": col,
        "end_line": line,
        "end_col": col + 1,
    }
