import sys, json, re, ast

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

def make_line_index(text):
    idx = []
    off = 0
    for ln, line in enumerate(text.splitlines(keepends=True), start=1):
        idx.append((ln, off, off+len(line)))
        off += len(line)
    return idx

def offset_to_line_col(line_index, offset):
    ln, col = 1, 1
    for (lno, s, e) in line_index:
        if s <= offset < e:
            ln = lno
            col = offset - s + 1
            break
    return ln, col

num_pat = re.compile(r"(?i)^\s*(?:(\d+)\s*'([bdho]))?\s*([0-9a-f_xz?]+)\s*$")

def parse_sized_number(tok):
    m = num_pat.match(tok)
    if not m:
        return None, None
    size, base, digits = m.groups()
    if base:
        base = base.lower()
    val = 0
    clean = re.sub(r"[xz?_]", "0", digits, flags=re.I)
    try:
        if base == 'b':
            val = int(clean, 2)
        elif base == 'd' or base is None:
            val = int(clean, 10)
        elif base == 'h':
            val = int(clean, 16)
        elif base == 'o':
            val = int(clean, 8)
    except Exception:
        return None, None
    width = int(size) if size is not None else None
    return val, width

expr_token_pat = re.compile(r"[A-Za-z_][A-Za-z0-9_$]*|[()]+|<<|>>|&&|\|\||==|!=|<=|>=|[~!%^&|+\-*/<>?:]|\d+(?:\s*'[bdho]\s*[0-9a-f_xz?]+)?", re.I)

def safe_eval_expr(expr, params):
    tokens = expr_token_pat.findall(expr)
    if not tokens:
        return None
    expr2 = []
    for t in tokens:
        v, _w = parse_sized_number(t)
        if v is not None:
            expr2.append(str(v))
            continue
        if re.fullmatch(r"[A-Za-z_][A-Za-z0-9_$]*", t):
            if t in params:
                expr2.append(str(params[t]))
            else:
                return None
        else:
            expr2.append(t)
    code = "".join(expr2)
    try:
        tree = ast.parse(code, mode="eval")
    except Exception:
        return None
    allowed = (ast.Expression, ast.BinOp, ast.UnaryOp, ast.Num, ast.operator, ast.Add, ast.Sub, ast.Mult, ast.Div, ast.FloorDiv, ast.Mod, ast.Pow,
               ast.LShift, ast.RShift, ast.BitOr, ast.BitAnd, ast.BitXor, ast.Invert, ast.USub, ast.UAdd, ast.Load, ast.Constant, ast.And, ast.Or,
               ast.Not, ast.Compare, ast.Eq, ast.NotEq, ast.Lt, ast.LtE, ast.Gt, ast.GtE, ast.Call, ast.Name)
    for node in ast.walk(tree):
        if not isinstance(node, allowed):
            return None
        if isinstance(node, ast.Call):
            return None
        if isinstance(node, ast.Name) and node.id not in params:
            return None
    try:
        val = eval(compile(tree, "<expr>", "eval"), {"__builtins__": {}}, {})
    except Exception:
        return None
    try:
        return int(val)
    except Exception:
        return None

def parse_params(text):
    out = {}
    pat = re.compile(r"\b(localparam|parameter)\b\s+([^;]+);", re.I)
    for m in pat.finditer(text):
        decls = m.group(2)
        for item in decls.split(","):
            item = item.strip()
            if "=" in item:
                name, expr = item.split("=", 1)
                name = name.strip().split()[-1]
                expr = expr.strip()
                val = safe_eval_expr(expr, out)
                if val is None:
                    continue
                out[name] = val
    return out

def parse_signal_widths(text, params):
    widths = {}
    decl_pat = re.compile(r"\b(?:logic|wire|reg|bit)\b\s*(\[[^\]]+\])?\s*([^;]+);", re.I)
    for m in decl_pat.finditer(text):
        rng = m.group(1)
        names = m.group(2)
        w = None
        if rng:
            m2 = re.match(r"\[\s*(.+?)\s*:\s*(.+?)\s*\]", rng)
            if m2:
                msb = safe_eval_expr(m2.group(1), params)
                lsb = safe_eval_expr(m2.group(2), params)
                if msb is not None and lsb is not None:
                    w = abs(msb - lsb) + 1
        for nm in names.split(","):
            nm = nm.strip()
            if not nm:
                continue
            nm = re.split(r"\s*=\s*", nm)[0].strip()
            nm = re.split(r"\s*\[", nm)[0].strip()
            if nm:
                widths[nm] = w
    port_pat = re.compile(r"\b(input|output|inout)\b\s+(?:wire|reg|logic|bit\s+)?(\[[^\]]+\])?\s*([^,);]+(?:\s*,\s*[^,);]+)*)", re.I)
    for m in port_pat.finditer(text):
        rng = m.group(2)
        names = m.group(3)
        w = None
        if rng:
            m2 = re.match(r"\[\s*(.+?)\s*:\s*(.+?)\s*\]", rng)
            if m2:
                msb = safe_eval_expr(m2.group(1), params)
                lsb = safe_eval_expr(m2.group(2), params)
                if msb is not None and lsb is not None:
                    w = abs(msb - lsb) + 1
        for nm in names.split(","):
            nm = nm.strip()
            nm = re.split(r"\s*=\s*", nm)[0].strip()
            nm = re.split(r"\s*\[", nm)[0].strip()
            nm = re.sub(r"\)$","", nm).strip()
            if nm:
                widths[nm] = widths.get(nm, w if w is not None else widths.get(nm))
    return widths

def width_of_expr(expr, params, widths):
    expr = expr.strip()
    if expr.startswith("(") and expr.endswith(")"):
        return width_of_expr(expr[1:-1], params, widths)
    m = re.match(r"^\{\s*(\d+)\s*\{\s*(.+)\s*\}\s*\}$", expr)
    if m:
        n = int(m.group(1))
        w = width_of_expr(m.group(2), params, widths)
        return None if w is None else n * w
    if expr.startswith("{") and expr.endswith("}"):
        inner = expr[1:-1]
        parts = []
        depth = 0
        cur = []
        for ch in inner:
            if ch == "{" or ch == "(":
                depth += 1
            elif ch == "}" or ch == ")":
                depth -= 1
            if ch == "," and depth == 0:
                parts.append("".join(cur).strip())
                cur = []
            else:
                cur.append(ch)
        if cur:
            parts.append("".join(cur).strip())
        total = 0
        for p in parts:
            w = width_of_expr(p, params, widths)
            if w is None:
                return None
            total += w
        return total
    m = re.match(r"^([A-Za-z_][A-Za-z0-9_$]*)\s*\[\s*(.+?)\s*:\s*(.+?)\s*\]$", expr)
    if m:
        name, msb, lsb = m.group(1), m.group(2), m.group(3)
        msb_v = safe_eval_expr(msb, params)
        lsb_v = safe_eval_expr(lsb, params)
        if msb_v is None or lsb_v is None:
            return None
        return abs(msb_v - lsb_v) + 1
    m = re.match(r"^([A-Za-z_][A-Za-z0-9_$]*)\s*\[\s*(.+?)\s*\]$", expr)
    if m:
        return 1
    val, w = parse_sized_number(expr)
    if val is not None:
        if w is not None:
            return w
        return None
    if re.fullmatch(r"[A-Za-z_][A-Za-z0-9_$]*", expr):
        return widths.get(expr)
    parts = re.split(r"[+\-|&^|*/<>]", expr)
    if len(parts) >= 2:
        ws = [width_of_expr(p, params, widths) for p in parts]
        if all(wi is not None for wi in ws):
            return max(ws)
    return None

assign_pat = re.compile(r"\bassign\b\s+(.+?)\s*=\s*(.+?);", re.I)

def find_width_mismatch_pp(text):
    viols = []
    line_index = make_line_index(text)
    params = parse_params(text)
    widths = parse_signal_widths(text, params)
    for m in assign_pat.finditer(text):
        lhs = m.group(1).strip()
        rhs = m.group(2).strip()
        wl = width_of_expr(lhs, params, widths)
        wr = width_of_expr(rhs, params, widths)
        if wl is not None and wr is not None and wl != wr:
            ln, col = offset_to_line_col(line_index, m.start(1))
            loc = {"line": ln, "col": col, "end_line": ln, "end_col": col + len(lhs)}
            viols.append(to_viol("width.mismatch", f"width mismatch: lhs={wl} vs rhs={wr}", loc))
    return viols

def find_unconnected_ports_pp(text):
    viols = []
    pat = re.compile(r'\.(?P<formal>[A-Za-z_][A-Za-z0-9_$]*)\s*\(\s*\)')
    line_index = make_line_index(text)
    for m in pat.finditer(text):
        ln, col = offset_to_line_col(line_index, m.start())
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
        violations.extend(find_width_mismatch_pp(txt))

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
