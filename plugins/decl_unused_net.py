import sys, json
def to_viol(rule_id, msg, loc, severity="warning"):
    return {"rule_id":rule_id,"severity":severity,"message":msg,"location":{"line":int(loc.get("line",1)),"col":int(loc.get("col",1)),"end_line":int(loc.get("end_line",1)),"end_col":int(loc.get("end_col",1))}}
def get_symbols(payload):
    s = payload.get("symbols")
    if s is None and isinstance(payload.get("ast"), dict):
        s = payload.get("ast", {}).get("symbols")
    if s is None:
        s = payload.get("symtab")
    if s is None:
        s = []
    return s
def is_net(sym):
    t = sym.get("class") or sym.get("kind") or sym.get("type")
    if isinstance(t, str) and t.lower() in ("net","wire"):
        return True
    if sym.get("is_net") is True:
        return True
    return False
def get_int(sym, keys, default=0):
    for k in keys:
        v = sym.get(k)
        if v is None:
            continue
        try:
            return int(v)
        except Exception:
            continue
    return default
def main():
    req = json.loads(sys.stdin.read() or "{}")
    stage = str(req.get("stage",""))
    payload = req.get("payload") or {}
    out = []
    for s in get_symbols(payload):
        if not is_net(s):
            continue
        r = get_int(s, ["read_count","reads","r"], 0)
        w = get_int(s, ["write_count","writes","w","ref_count"], 0)
        if r == 0 and w == 0:
            name = s.get("name","")
            loc = s.get("loc") or {"line":1,"col":1,"end_line":1,"end_col":1}
            out.append(to_viol("decl.unused.net", "'{}' declared but never used".format(name), loc))
    sys.stdout.write(json.dumps({"type":"ViolationsStage","stage":stage,"violations":out}))
if __name__ == "__main__":
    main()
