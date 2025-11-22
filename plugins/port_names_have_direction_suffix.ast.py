def check(req):
    if req.get("stage") != "ast":
        return []
    payload = req.get("payload") or {}
    ports = payload.get("ports") or []
    out = []
    suffixes = {
        "input": ("_i", "_ni"),
        "output": ("_o", "_no"),
        "inout": ("_io", "_nio"),
    }
    for port in ports:
        direction = (port.get("direction") or "").lower()
        allowed = suffixes.get(direction)
        if not allowed:
            continue
        name = port.get("name") or ""
        if not name or any(name.endswith(sfx) for sfx in allowed):
            continue
        loc = port.get("loc") or {"line": 1, "col": 1, "end_line": 1, "end_col": 1}
        out.append({
            "rule_id": "port_names_have_direction_suffix",
            "severity": "warning",
            "message": f"{name} must end with {' or '.join(allowed)}",
            "location": loc,
        })
    return out
