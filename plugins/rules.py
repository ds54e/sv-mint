import sys, json

def respond(stage, violations):
    out = {
        "type": "ViolationsStage",
        "stage": stage,
        "violations": violations,
    }
    sys.stdout.write(json.dumps(out))
    sys.stdout.flush()

def handle_ast(req):
    ast = req.get("payload", {}).get("ast", {})
    decls = {d.get("name") for d in ast.get("declarations", []) if d.get("name")}
    used = {r.get("name") for r in ast.get("references", []) if r.get("name") and r.get("kind") in ("Rhs", "Lhs")}
    unused = sorted(n for n in decls - used if n)
    viol = []
    for n in unused:
        viol.append({
            "rule_id": "decl.unused",
            "severity": "warning",
            "message": f"'{n}' declared but never used",
            "location": {"line": 1, "col": 1, "end_line": 1, "end_col": 1}
        })
    return viol

def main():
    req = json.load(sys.stdin)
    if req.get("type") != "CheckFileStage":
        respond(req.get("stage") or "raw_text", [])
        return
    stage = req.get("stage")
    if stage == "ast":
        violations = handle_ast(req)
    else:
        violations = []
    respond(stage, violations)

if __name__ == "__main__":
    main()
