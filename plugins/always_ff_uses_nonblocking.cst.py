from lib.cst_inline import Cst, byte_span_to_loc


def check(req):
    if req.get("stage") != "cst":
        return []
    payload = req.get("payload") or {}
    ir = payload.get("cst_ir") or {}
    cst = Cst(ir)
    line_starts = ir.get("line_starts") or [0]
    tokens = ir.get("tokens") or []
    kinds = {name: i for i, name in enumerate(ir.get("tok_kind_table") or [])}
    op_eq = kinds.get("op_eq")
    kw_ff = kinds.get("kw_always_ff")
    regions = []
    if cst.nodes and tokens and kw_ff is not None:
        for node in cst.of_kind("AlwaysConstruct"):
            toks = cst.tokens_in(node)
            if any(t.get("kind") == kw_ff for t in toks):
                regions.append((node.get("start"), node.get("end")))
    out = []
    if tokens and op_eq is not None and regions:
        for start, end in regions:
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
