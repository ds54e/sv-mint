from lib.cst_inline import byte_span_to_loc


def check(req):
    if req.get("stage") != "cst":
        return []
    payload = req.get("payload") or {}
    if payload.get("mode") != "inline":
        return []
    ir = payload.get("cst_ir") or {}
    tokens = ir.get("tokens") or []
    tok_names = ir.get("tok_kind_table") or []
    line_starts = ir.get("line_starts") or [0]
    kinds = {name: idx for idx, name in enumerate(tok_names)}
    wildcard_kind = kinds.get("conn_wildcard")
    if wildcard_kind is None:
        return []
    out = []
    for tok in tokens:
        if tok.get("kind") != wildcard_kind:
            continue
        start = tok.get("start")
        end = tok.get("end")
        if start is None or end is None:
            continue
        loc = byte_span_to_loc(start, end, line_starts)
        out.append({
            "rule_id": "module.no_port_wildcard",
            "severity": "warning",
            "message": "named port connections must not use .* wildcard",
            "location": loc,
        })
    return out
