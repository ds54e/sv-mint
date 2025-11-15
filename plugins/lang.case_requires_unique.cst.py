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
        case_kind = kinds.index("kw_case")
    except ValueError:
        return []
    unique_kind = kinds.index("kw_unique") if "kw_unique" in kinds else None
    priority_kind = kinds.index("kw_priority") if "kw_priority" in kinds else None
    out = []
    for node in cst.of_kind("CaseStatement"):
        toks = cst.tokens_in(node)
        for idx, tok in enumerate(toks):
            if tok.get("kind") != case_kind:
                continue
            has_prefix = False
            for prev in reversed(toks[:idx]):
                kind = prev.get("kind")
                if kind == unique_kind or kind == priority_kind:
                    has_prefix = True
                    break
                if kind == case_kind:
                    break
            if not has_prefix:
                loc = byte_span_to_loc(tok.get("start"), tok.get("end"), line_starts)
                out.append({
                    "rule_id": "lang.case_requires_unique",
                    "severity": "warning",
                    "message": "case statements should use unique or priority",
                    "location": loc,
                })
            break
    return out
