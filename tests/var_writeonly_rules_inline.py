import sys, json, re

def to_viol(rule_id, msg, loc, severity="warning"):
    return {
        "rule_id": rule_id,
        "severity": severity,
        "message": msg,
        "location": {
            "line": int(loc.get("line", 1)),
            "col": int(loc.get("col", 1)),
            "end_line": int(loc.get("end_line", 1)),
            "end_col": int(loc.get("end_col", 1)),
        },
    }

def find_unconnected_ports_pp(text):
    viols = []
    pat = re.compile(r'\.(?P<formal>[A-Za-z_][A-Za-z0-9_$]*)\s*\(\s*\)')
    lines = text.splitlines(keepends=True)
    offsets = []
    off = 0
    for ln, s in enumerate(lines, start=1):
        offsets.append((ln, off, off + len(s)))
        off += len(s)
    for m in pat.finditer(text):
        start = m.start()
        ln, col = 1, 1
        for (lno, s, e) in offsets:
            if s <= start < e:
                ln = lno
                col = start - s + 1
                break
        name = m.group("formal")
        loc = {"line": ln, "col": col, "end_line": ln, "end_col": col + len(name) + 3}
        viols.append(to_viol("port.unconnected", f"port '{name}' connected as empty", loc))
    return viols

def main():
    data = sys.stdin.buffer.read()
    req = json.loads(data.decode("utf-8")) if data else {}
    stage = req.get("stage")
    payload = req.get("payload") or {}
    violations = []

    if stage == "pp_text":
        txt = payload.get("text") or ""
        violations.extend(find_unconnected_ports_pp(txt))

    if stage == "ast":
        symbols = payload.get("symbols") or []
        for s in symbols:
            cls = s.get("class")
            name = s.get("name") or "?"
            loc = s.get("loc") or {"line":1,"col":1,"end_line":1,"end_col":1}
            r = int(s.get("read_count", 0))
            w = int(s.get("write_count", s.get("ref_count", 0)))
            if cls in ("param", "net", "var"):
                if r == 0 and w == 0:
                    violations.append(to_viol("decl.unused", f"'{name}' declared but never used", loc))
            if cls == "var":
                if w > 0 and r == 0:
                    violations.append(to_viol("var.writeonly", f"'{name}' written but never read", loc))

    resp = {"type": "ViolationsStage", "stage": stage, "violations": violations}
    sys.stdout.write(json.dumps(resp))

if __name__ == "__main__":
    main()
