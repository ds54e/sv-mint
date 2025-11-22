def check(req):
    if req.get("stage") != "ast":
        return []
    payload = req.get("payload") or {}
    decls = payload.get("decls") or []
    out = []
    for decl in decls:
        if decl.get("kind") != "module":
            continue
        name = decl.get("name") or ""
        loc = decl.get("loc")
        if loc is None:
            continue
        if not _lower_snake(name):
            out.append(
                {
                    "rule_id": "module_names_lower_snake",
                    "severity": "warning",
                    "message": f"{name} must use lower_snake_case",
                    "location": loc,
                }
            )
    return out

def _lower_snake(name):
    return name and name[0].islower() and all(c.isalnum() or c == "_" for c in name)
