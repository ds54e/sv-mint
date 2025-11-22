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
    for node in cst.of_kind("AlwaysConstruct"):
        fields = node.get("fields") or {}
        for ev in fields.get("events") or []:
            if (ev.get("separator") or "").lower() != "or":
                continue
            tok = ev.get("token")
            if tok is None or tok >= len(tokens):
                continue
            start = tokens[tok].get("start")
            end = tokens[tok].get("end")
            if start is None or end is None:
                continue
            loc = byte_span_to_loc(start, end, line_starts)
            out.append(
                {
                    "rule_id": "sensitivity_list_uses_commas",
                    "severity": "warning",
                    "message": "use ',' separators in sensitivity lists instead of 'or'",
                    "location": loc,
                }
            )
    return out
