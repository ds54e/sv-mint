from lib.cst_inline import byte_span_to_loc

def check(req):
    if req.get("stage") != "cst":
        return []
    payload = req.get("payload") or {}
    ir = payload.get("cst_ir") or {}
    directives = ir.get("directives") or []
    line_starts = ir.get("line_starts") or [0]
    tokens = ir.get("tokens") or []
    defaults = [
        d for d in directives if (d.get("kind") or "").lower() == "default_nettype"
    ]
    if not defaults:
        return []
    last = defaults[-1]
    value = (last.get("value") or "").lower()
    if value == "wire":
        return []
    tok = last.get("token")
    if tok is not None and tok < len(tokens):
        loc = byte_span_to_loc(
            tokens[tok].get("start"), tokens[tok].get("end"), line_starts
        )
    else:
        start = last.get("start") or 0
        end = last.get("end") or start
        loc = byte_span_to_loc(start, end, line_starts)
    return [
        {
            "rule_id": "default_nettype_ends_with_wire",
            "severity": "warning",
            "message": "`default_nettype none` should be reset to `wire` at the end of the file",
            "location": loc,
        }
    ]
