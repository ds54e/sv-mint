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
        for idx in _or_tokens_in_sensitivity(tokens, first, last, text):
            start = tokens[idx].get("start")
            end = tokens[idx].get("end")
            if start is None or end is None:
                continue
            loc = byte_span_to_loc(start, end, line_starts)
            out.append({
                "rule_id": "sensitivity_list_uses_commas",
                "severity": "warning",
                "message": "use ',' separators in sensitivity lists instead of 'or'",
                "location": loc,
            })
    return out


def _or_tokens_in_sensitivity(tokens, first, last, text):
    i = first
    n = last + 1
    while i < n:
        tok = tokens[i]
        word = _tok_text(tok, text)
        if word != "@":
            i += 1
            continue
        j = i + 1
        depth = 0
        while j < n:
            word_j = _tok_text(tokens[j], text)
            if word_j == "(":
                depth += 1
            elif word_j == ")":
                if depth == 0:
                    break
                depth -= 1
                if depth == 0:
                    break
            elif depth > 0 and word_j.lower() == "or":
                yield j
            j += 1
        i = j + 1


def _tok_text(tok, text):
    start = tok.get("start")
    end = tok.get("end")
    if start is None or end is None:
        return ""
    return text[start:end].strip()
