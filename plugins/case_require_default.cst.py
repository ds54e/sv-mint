from lib.cst_inline import Cst, byte_span_to_loc


def check(req):
    if req.get("stage") != "cst":
        return []
    payload = req.get("payload") or {}
    if payload.get("mode") != "inline":
        return []
    ir = payload.get("cst_ir") or {}
    cst = Cst(ir)
    tokens = ir.get("tokens") or []
    line_starts = ir.get("line_starts") or [0]
    text = ir.get("pp_text") or ""
    out = []
    for node in cst.of_kind("CaseStatement"):
        first = node.get("first_token")
        last = node.get("last_token")
        if first is None or last is None:
            continue
        has_default = False
        is_unique = False
        for tok in tokens[first:last + 1]:
            start = tok.get("start")
            end = tok.get("end")
            if start is None or end is None:
                continue
            word = text[start:end].strip().lower()
            if word == "default":
                has_default = True
                break
            if word in ("unique", "unique0"):
                is_unique = True
        if not has_default and not is_unique:
            anchor = tokens[first]
            loc = byte_span_to_loc(anchor.get("start"), anchor.get("end"), line_starts)
            out.append({
                "rule_id": "case_require_default",
                "severity": "warning",
                "message": "case statement must include a default item",
                "location": loc,
            })
    return out
