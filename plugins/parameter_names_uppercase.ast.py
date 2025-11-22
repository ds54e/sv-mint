def check(req):
    if req.get("stage") != "ast":
        return []
    payload = req.get("payload") or {}
    decls = payload.get("decls") or []
    out = []
    for decl in decls:
        if decl.get("kind") != "param":
            continue
        name = decl.get("name") or ""
        if not name:
            continue
        if _is_upper(name):
            continue
        loc = decl.get("loc") or {"line": 1, "col": 1, "end_line": 1, "end_col": 1}
        out.append({
            "rule_id": "parameter_names_uppercase",
            "severity": "warning",
            "message": f"parameter {name} should use UpperCamelCase or ALL_CAPS",
            "location": loc,
        })
    return out


def _is_upper(name):
    if not name:
        return False
    if name[0].isupper() and name.replace("_", "").isalnum():
        return True
    if name.isupper():
        return True
    return False
