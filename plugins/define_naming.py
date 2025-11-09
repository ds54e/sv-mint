#!/usr/bin/env python3
import sys, json, re

DEFINE_RE = re.compile(r'^\s*`define\s+([A-Za-z_][A-Za-z0-9_]*)')

def main():
    try:
        req = json.loads(sys.stdin.read())
    except Exception:
        return
    text = req.get("text", "")
    violations = []
    for i, line in enumerate(text.splitlines(), 1):
        m = DEFINE_RE.match(line)
        if m and not m.group(1).isupper():
            violations.append({
                "rule_id": "naming.define_upper",
                "severity": "error",
                "message": f"`define '{m.group(1)}' should be UPPER_CASE",
                "location": {"line": i, "col": 1, "end_line": i, "end_col": len(line)}
            })
    resp = {"type": "ViolationsSingle", "violations": violations}
    sys.stdout.write(json.dumps(resp, ensure_ascii=False) + "\n")
    sys.stdout.flush()

if __name__ == "__main__":
    main()
