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
    kinds = ir.get("tok_kind_table") or []
    line_starts = ir.get("line_starts") or [0]
    try:
        colon_kind = kinds.index("colon")
    except ValueError:
        return []
    begin_kind = kinds.index("kw_begin") if "kw_begin" in kinds else None
    if begin_kind is None:
        return []
    out = []
    for node in cst.of_kind("CaseStatement"):
        node_tokens = cst.tokens_in(node)
        total = len(node_tokens)
        for idx, tok in enumerate(node_tokens):
            if tok.get("kind") != colon_kind:
                continue
            nxt = next_non_comment(node_tokens, idx + 1)
            if nxt is None:
                continue
            if nxt.get("kind") == begin_kind:
                continue
            loc = byte_span_to_loc(tok.get("start"), tok.get("end"), line_starts)
            out.append({
                "rule_id": "format.case_begin_required",
                "severity": "warning",
                "message": "case item should wrap statements in begin/end",
                "location": loc,
            })
    return out


def next_non_comment(tokens, start):
    for tok in tokens[start:]:
        kind = tok.get("kind")
        if kind is None:
            continue
        # skip comments classified via classify_token
        if kind in ("line_comment", "block_comment"):
            continue
        return tok
    return None
