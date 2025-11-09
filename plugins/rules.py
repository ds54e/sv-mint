#!/usr/bin/env python3
import sys, json, re

def build_regex(pattern, allow_leading_underscore):
    if pattern:
        return re.compile(pattern)
    if allow_leading_underscore:
        return re.compile(r"^_?[A-Z][A-Z0-9_]*$")
    return re.compile(r"^[A-Z][A-Z0-9_]*$")

def main():
    try:
        req = json.loads(sys.stdin.read())
    except Exception:
        return

    if req.get("type") != "CheckFileStage":
        return

    stage = req.get("stage")
    if stage not in ("pp_text", "raw_text", "cst"):
        return

    payload = req.get("payload", {})
    violations = []

    if stage == "pp_text":
        rules = payload.get("rules", {}).get("define_upper", {})
        pattern = rules.get("pattern", None)
        allow_leading_underscore = rules.get("allow_leading_underscore", False)
        include_predefines = rules.get("include_predefines", False)
        regex = build_regex(pattern, allow_leading_underscore)

        names = list(payload.get("defines_table", []))
        predefined = set(payload.get("predefined_names", []))

        if not include_predefines:
            names = [n for n in names if n not in predefined]

        meta = {m.get("name"): m for m in payload.get("defines_table_meta", [])}

        for name in names:
            if not regex.match(name):
                m = meta.get(name, {})
                line = m.get("line", 1)
                col = m.get("col", 1)
                violations.append({
                    "rule_id": "naming.define_upper",
                    "severity": "error",
                    "message": f"`define '{name}' should match pattern",
                    "location": {
                        "line": line,
                        "col": col,
                        "end_line": line,
                        "end_col": col
                    }
                })

    resp = {
        "type": "ViolationsStage",
        "stage": stage,
        "violations": violations
    }

    sys.stdout.write(json.dumps(resp, ensure_ascii=False) + "\n")
    sys.stdout.flush()

if __name__ == "__main__":
    main()
