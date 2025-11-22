from lib.cst_inline import Cst, byte_span_to_loc


def check(req):
    if req.get("stage") != "cst":
        return []
    payload = req.get("payload") or {}
    ir = payload.get("cst_ir") or {}
    cst = Cst(ir)
    line_starts = ir.get("line_starts") or [0]
    tokens = ir.get("tokens") or []
    tok_kinds = ir.get("tok_kind_map") or {}
    op_eq = tok_kinds.get("op_eq")
    out = []
    if tokens and op_eq is not None:
        for node in cst.of_kind("AlwaysConstruct"):
            fields = node.get("fields") or {}
            if (fields.get("always_kind") or "").lower() != "ff":
                continue
            start = node.get("start")
            end = node.get("end")
            if start is None or end is None:
                continue
            for tok in tokens:
                ts = tok.get("start")
                te = tok.get("end")
                if ts is None or te is None:
                    continue
                if te <= start:
                    continue
                if ts >= end:
                    break
                if tok.get("kind") == op_eq:
                    loc = byte_span_to_loc(ts, te, line_starts)
                    out.append({
                        "rule_id": "always_ff_uses_nonblocking",
                        "severity": "warning",
                        "message": "blocking '=' inside always_ff",
                        "location": loc,
                    })
    return out
