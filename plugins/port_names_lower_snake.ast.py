def check(req):
    if req.get("stage") != "ast":
        return []
    payload = req.get("payload") or {}
    ports = payload.get("ports") or []
    out = []
    for port in ports:
        name = port.get("name") or ""
        loc = port.get("loc")
        if loc is None:
            continue
        if not _lower_snake(name):
            out.append(
                {
                    "rule_id": "port_names_lower_snake",
                    "severity": "warning",
                    "message": f"{name} must use lower_snake_case",
                    "location": loc,
                }
            )
    return out

def _lower_snake(name):
    return name and name[0].islower() and all(c.isalnum() or c == "_" for c in name)
