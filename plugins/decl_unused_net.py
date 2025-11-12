import sys, json
req = json.load(sys.stdin)
stage = req.get("stage")
if stage != "ast":
    print(json.dumps({"type":"ViolationsStage","stage":stage,"violations":[]}))
    sys.exit(0)
p = req.get("payload", {})
syms = p.get("symbols", [])
vs = []
for s in syms:
    if s.get("class") != "net":
        continue
    rc = int(s.get("read_count", 0) or 0)
    wc = int(s.get("write_count", 0) or 0)
    if rc == 0 and wc == 0:
        vs.append({
            "rule_id":"decl.unused.net",
            "severity":"warning",
            "message":f"unused net {s.get('module','')}.{s.get('name','')}",
            "location":s.get("loc", {"line":1,"col":1,"end_line":1,"end_col":1})
        })
print(json.dumps({"type":"ViolationsStage","stage":"ast","violations":vs}))
