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
    for node in cst.of_kind("AlwaysConstruct"):
        first = node.get("first_token")
        last = node.get("last_token")
        if first is None or last is None:
            continue
        word = _first_word(tokens, first, last, text)
        if word in ("always_ff", "always_comb", "always_latch"):
            continue
        start = tokens[first].get("start")
        end = tokens[first].get("end")
        if start is None or end is None:
            continue
        loc = byte_span_to_loc(start, end, line_starts)
        out.append({
            "rule_id": "always_is_structured",
            "severity": "warning",
            "message": "use always_ff/always_comb/always_latch instead of bare always",
            "location": loc,
        })
    return out


def _first_word(tokens, first, last, text):
    for tok in tokens[first:last + 1]:
        start = tok.get("start")
        end = tok.get("end")
        if start is None or end is None:
            continue
        return text[start:end].strip().lower()
    return ""
