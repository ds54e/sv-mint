from lib.cst_inline import byte_span_to_loc


def check(req):
    if req.get("stage") != "cst":
        return []
    payload = req.get("payload") or {}
    ir = payload.get("cst_ir") or {}
    directives = ir.get("directives") or []
    line_starts = ir.get("line_starts") or [0]
    tokens = ir.get("tokens") or []
    defaults = [d for d in directives if (d.get("kind") or "").lower() == "default_nettype"]
    if defaults:
        return []
    loc = byte_span_to_loc(0, 0, line_starts)
    return [{
        "rule_id": "default_nettype_begins_with_none",
        "severity": "warning",
        "message": "file must declare `default_nettype none` near the top",
        "location": loc,
    }]
