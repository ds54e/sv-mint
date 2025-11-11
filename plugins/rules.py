import sys, json

def main():
    data = sys.stdin.buffer.read()
    if not data:
        req = {}
    else:
        try:
            req = json.loads(data.decode("utf-8"))
        except Exception:
            req = {}
    stage = req.get("stage")
    payload = req.get("payload") or {}
    violations = []

    if stage == "ast":
        symbols = payload.get("symbols")
        if symbols is None:
            symbols = []
            decls = payload.get("decls") or []
            refs = payload.get("refs") or []
            ref_counts = {}
            for r in refs:
                n = r.get("name")
                m = r.get("module","")
                if n:
                    ref_counts[(m,n)] = ref_counts.get((m,n),0) + 1
            for d in decls:
                k = d.get("kind")
                if k in ("param","net","var"):
                    n = d.get("name")
                    m = d.get("module","")
                    loc = d.get("loc") or {"line":1,"col":1,"end_line":1,"end_col":1}
                    c = ref_counts.get((m,n),0)
                    symbols.append({"module":m,"name":n,"class":k,"ref_count":c,"used":c>0,"loc":loc})
        for s in symbols:
            cls = s.get("class")
            if cls in ("param","net","var"):
                used = s.get("used")
                ref_count = s.get("ref_count", 0)
                if (used is False) or (isinstance(ref_count, int) and ref_count == 0):
                    name = s.get("name") or "?"
                    loc = s.get("loc") or {"line":1,"col":1,"end_line":1,"end_col":1}
                    violations.append({
                        "rule_id": "decl.unused",
                        "severity": "warning",
                        "message": f"'{name}' declared but never used",
                        "location": {
                            "line": int(loc.get("line",1)),
                            "col": int(loc.get("col",1)),
                            "end_line": int(loc.get("end_line",1)),
                            "end_col": int(loc.get("end_col",1))
                        }
                    })

    resp = {"type": "ViolationsStage", "stage": stage, "violations": violations}
    sys.stdout.write(json.dumps(resp))

if __name__ == "__main__":
    main()
