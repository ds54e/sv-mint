import re
from lib.cst_inline import Cst, byte_span_to_loc


def find_regions_text(text):
    out = []
    i = 0
    n = len(text)
    while True:
        i = text.find("always_comb", i)
        if i < 0:
            break
        j = i
        depth = 0
        while j < n:
            if text.startswith("begin", j):
                depth += 1
                j += 5
                continue
            if text.startswith("end", j):
                if depth == 0:
                    out.append((i, j + 3))
                    break
                depth -= 1
                j += 3
                continue
            if text[j] == ';' and depth == 0:
                out.append((i, j + 1))
                break
            j += 1
        i += 1
    return out


def check(req):
    if req.get("stage") != "cst":
        return []
    payload = req.get("payload") or {}
    ir = payload.get("cst_ir") or {}
    if not ir:
        return []
    cst = Cst(ir)
    text = ir.get("pp_text") or ""
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
    else:
        regions = find_regions_text(text)
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
                        "rule_id": "comb.no_nb_in_always_comb",
                        "severity": "warning",
                        "message": "nonblocking '<=' inside always_comb",
                        "location": loc,
                    })
    else:
        pat = re.compile(r"<=")
        for start, end in regions:
            scan = start
            while True:
                m = pat.search(text, scan, end)
                if not m:
                    break
                loc = byte_span_to_loc(m.start(), m.end(), line_starts)
                out.append({
                    "rule_id": "comb.no_nb_in_always_comb",
                    "severity": "warning",
                    "message": "nonblocking '<=' inside always_comb",
                    "location": loc,
                })
                scan = m.end()
    return out
