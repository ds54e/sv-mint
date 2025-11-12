import sys, json, re
from lib.cst_inline import Cst, byte_span_to_loc

def find_regions_text(text):
    out = []
    i = 0
    n = len(text)
    while True:
        i = text.find("always_ff", i)
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

def main():
    req = json.loads(sys.stdin.read() or "{}")
    if req.get("stage") != "cst":
        print(json.dumps({"type":"ViolationsStage","stage":req.get("stage"),"violations":[]}))
        return
    p = req.get("payload") or {}
    ir = p.get("cst_ir") or {}
    if not ir:
        print(json.dumps({"type":"ViolationsStage","stage":"cst","violations":[]}))
        return
    cst = Cst(ir)
    text = ir.get("pp_text") or ""
    line_starts = ir.get("line_starts") or [0]
    viol = []
    TK = {name:i for i,name in enumerate(ir.get("tok_kind_table") or [])}
    op_eq = TK.get("op_eq")
    kw_ff = TK.get("kw_always_ff")
    regions = []
    if cst.nodes and cst.tokens and kw_ff is not None:
        for n in cst.of_kind("AlwaysConstruct"):
            toks = cst.tokens_in(n)
            if any(t.get("kind") == kw_ff for t in toks):
                regions.append((n.get("start"), n.get("end")))
    else:
        regions = find_regions_text(text)
    if cst.tokens and op_eq is not None and regions:
        toks = ir.get("tokens") or []
        for S,E in regions:
            for t in toks:
                ts, te, k = t.get("start"), t.get("end"), t.get("kind")
                if te <= S:
                    continue
                if ts >= E:
                    break
                if k == op_eq:
                    loc = byte_span_to_loc(ts, te, line_starts)
                    viol.append({"rule_id":"seq.blocking_in_alwaysff","severity":"warning","message":"blocking '=' inside always_ff","location":loc})
    else:
        pat = re.compile(r"(?<!<)=(?!=)")
        for S,E in regions:
            i = S
            while True:
                m = pat.search(text, i, E)
                if not m:
                    break
                s, e = m.start(), m.end()
                loc = byte_span_to_loc(s, e, line_starts)
                viol.append({"rule_id":"seq.blocking_in_alwaysff","severity":"warning","message":"blocking '=' inside always_ff","location":loc})
                i = e
    print(json.dumps({"type":"ViolationsStage","stage":"cst","violations":viol}))

if __name__ == "__main__":
    main()
