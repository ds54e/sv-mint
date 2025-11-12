def check(req):
    if req.get("stage") != "ast":
        return []
    payload = req.get("payload") or {}
    symbols = payload.get("symbols") or []
    out = []
    for s in symbols:
        if s.get("class") != "var":
            continue
        reads = int(s.get("read_count", 0) or 0)
        writes = int(s.get("write_count", 0) or 0)
        if reads == 0 and writes == 0:
            out.append({
                "rule_id": "decl.unused.var",
                "severity": "warning",
                "message": f"unused var {s.get('module','')}.{s.get('name','')}",
                "location": s.get("loc", {"line":1,"col":1,"end_line":1,"end_col":1})
            })
    return out
