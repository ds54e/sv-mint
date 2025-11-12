def to_viol(rule_id, msg, severity="warning"):
    return {
        "rule_id": rule_id,
        "severity": severity,
        "message": msg,
        "location": {"line": 1, "col": 1, "end_line": 1, "end_col": 1},
    }


def collect_symbols(payload):
    symbols = payload.get("symbols")
    if symbols is None:
        ast = payload.get("ast")
        if isinstance(ast, dict):
            symbols = ast.get("symbols")
    if symbols is None:
        symbols = payload.get("symtab")
    if symbols is None:
        symbols = []
    return symbols


def check(req):
    stage = str(req.get("stage", ""))
    payload = req.get("payload") or {}
    symbols = collect_symbols(payload)
    msg = f"debug ping: stage={stage}, symbols={len(symbols)}"
    return [to_viol("debug.ping", msg)]
