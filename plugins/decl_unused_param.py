def check(req):
    if req.get("stage") != "ast":
        return []
    payload = req.get("payload") or {}
    symbols = payload.get("symbols") or []
    out = []
    for s in symbols:
        if s.get("class") != "param":
            continue
        refs = int(s.get("ref_count", s.get("read_count", 0) or 0) or 0)
        if refs == 0:
            out.append({
                "rule_id": "decl.unused.param",
                "severity": "warning",
                "message": f"unused param {s.get('module','')}.{s.get('name','')}",
                "location": s.get("loc", {"line":1,"col":1,"end_line":1,"end_col":1})
            })
    return out
