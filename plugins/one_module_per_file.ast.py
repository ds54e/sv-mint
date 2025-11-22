def check(req):
    if req.get("stage") != "ast":
        return []
    payload = req.get("payload") or {}
    decls = payload.get("decls") or []
    modules = [d for d in decls if d.get("kind") == "module"]
    if len(modules) <= 1:
        return []
    out = []
    for m in modules[1:]:
        out.append(
            {
                "rule_id": "one_module_per_file",
                "severity": "warning",
                "message": "file must contain only one module declaration",
                "location": m.get("loc"),
            }
        )
    return out
