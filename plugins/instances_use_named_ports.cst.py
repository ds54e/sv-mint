from lib.cst_inline import Cst, byte_span_to_loc


def check(req):
    if req.get("stage") != "cst":
        return []
    payload = req.get("payload") or {}
    ir = payload.get("cst_ir") or {}
    cst = Cst(ir)
    tokens = ir.get("tokens") or []
    line_starts = ir.get("line_starts") or [0]
    out = []
    for node in cst.of_kind("HierarchicalInstance"):
        fields = node.get("fields") or {}
        conns = fields.get("connections") or []
        if not conns:
            continue
        if any(not (c.get("named") is True) for c in conns):
            tok = fields.get("name_token") or node.get("first_token")
            if tok is None or tok >= len(tokens):
                continue
            loc = byte_span_to_loc(
                tokens[tok].get("start"),
                tokens[tok].get("end"),
                line_starts,
            )
            out.append({
                "rule_id": "instances_use_named_ports",
                "severity": "warning",
                "message": "use named port connections instead of positional arguments",
                "location": loc,
            })
    return out
