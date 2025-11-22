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
    op_le = kinds.get("op_le")
    kw_comb = kinds.get("kw_always_comb")
    regions = []
    if cst.nodes and tokens and kw_comb is not None:
        for node in cst.of_kind("AlwaysConstruct"):
            toks = cst.tokens_in(node)
            if any(t.get("kind") == kw_comb for t in toks):
                regions.append((node.get("start"), node.get("end")))
    out = []
    if tokens and op_le is not None and regions:
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
                if tok.get("kind") == op_le:
                    loc = byte_span_to_loc(ts, te, line_starts)
                    out.append({
                        "rule_id": "always_comb_uses_blocking",
                        "severity": "warning",
                        "message": "nonblocking '<=' inside always_comb",
                        "location": loc,
                    })
    return out
