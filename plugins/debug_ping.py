import sys, json
def to_viol(rule_id, msg, loc, severity="warning"):
    return {"rule_id":rule_id,"severity":severity,"message":msg,"location":{"line":1,"col":1,"end_line":1,"end_col":1}}
def get_symbols(payload):
    s = payload.get("symbols")
    if s is None and isinstance(payload.get("ast"), dict):
        s = payload.get("ast", {}).get("symbols")
    if s is None:
        s = payload.get("symtab")
    if s is None:
        s = []
    return s
def main():
    req = json.loads(sys.stdin.read() or "{}")
    stage = str(req.get("stage",""))
    payload = req.get("payload") or {}
    syms = get_symbols(payload)
    msg = "debug ping: stage={}, symbols={}".format(stage, len(syms))
    viol = to_viol("debug.ping", msg, {})
    sys.stdout.write(json.dumps({"type":"ViolationsStage","stage":stage,"violations":[viol]}))
if __name__ == "__main__":
    main()
