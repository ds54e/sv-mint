def check(req):
    if req.get("stage") != "ast":
        return []
    payload = req.get("payload") or {}
    assigns = payload.get("assigns") or []
    buckets = {}
    for assign in assigns:
        if assign.get("op") != "nonblocking":
            continue
        key = (assign.get("module"), assign.get("lhs"))
        buckets.setdefault(key, []).append(assign)
    out = []
    for entries in buckets.values():
        if len(entries) <= 1:
            continue
        for entry in entries[1:]:
            out.append({
                "rule_id": "flow.no_multiple_nb_assign",
                "severity": "warning",
                "message": f"multiple nonblocking assignments to {entry.get('lhs')}",
                "location": entry.get("loc"),
            })
    return out
