import sys, json
data = sys.stdin.read()
req = json.loads(data) if data else {}
viol = []
stage = req.get("stage")
payload = req.get("payload") or {}
if stage == "raw_text":
    txt = payload.get("text") or ""
    if "unused" in txt:
        viol.append({"rule_id":"decl.unused","severity":"warning","message":"declared but never used","location":{"line":1,"col":1,"end_line":1,"end_col":1}})
resp = {"type":"ViolationsStage","stage":stage,"violations":viol}
sys.stdout.write(json.dumps(resp))
