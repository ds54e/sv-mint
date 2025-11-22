from lib.cst_inline import Cst, byte_span_to_loc

def check(req):
    if req.get("stage") != "cst":
        return []
    payload = req.get("payload") or {}
    ir = payload.get("cst_ir") or {}
    cst = Cst(ir)
    line_starts = ir.get("line_starts") or [0]
    out = []
    for node in cst.of_kind("AlwaysConstruct"):
        fields = node.get("fields") or {}
        kind = (fields.get("always_kind") or "always").lower()
        if kind in ("ff", "comb", "latch"):
            continue
        tokens = ir.get("tokens") or []
        first = node.get("first_token")
        if first is None:
            continue
        start = tokens[first].get("start")
        end = tokens[first].get("end")
        if start is None or end is None:
            continue
        loc = byte_span_to_loc(start, end, line_starts)
        out.append(
            {
                "rule_id": "always_is_structured",
                "severity": "warning",
                "message": "use always_ff/always_comb/always_latch instead of bare always",
                "location": loc,
            }
        )
    return out
