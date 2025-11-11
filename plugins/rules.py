import sys, json

def main():
    try:
        data = sys.stdin.buffer.read()
        req = json.loads(data.decode("utf-8")) if data else {}
    except Exception:
        req = {}
    stage = req.get("stage")
    payload = req.get("payload") or {}
    violations = []

    if stage == "ast":
        symbols = payload.get("symbols") or []
        for s in symbols:
            cls = s.get("class")
            if cls in ("param", "net", "var"):
                used = s.get("used")
                ref_count = s.get("ref_count", 0)
                if (used is False) or (isinstance(ref_count, int) and ref_count == 0):
                    name = s.get("name") or "?"
                    violations.append({
                        "rule_id": "decl.unused",
                        "severity": "warning",
                        "message": f"'{name}' declared but never used",
                        "location": {"line": 1, "col": 1, "end_line": 1, "end_col": 1}
                    })

    resp = {"type": "ViolationsStage", "stage": stage, "violations": violations}
    sys.stdout.write(json.dumps(resp))

if __name__ == "__main__":
    main()
