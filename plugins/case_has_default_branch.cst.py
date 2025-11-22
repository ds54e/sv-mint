from lib.cst_inline import Cst, byte_span_to_loc


def check(req):
    if req.get("stage") != "cst":
        return []
    payload = req.get("payload") or {}
    ir = payload.get("cst_ir") or {}
    cst = Cst(ir)
    tokens = ir.get("tokens") or []
    line_starts = ir.get("line_starts") or [0]
    out = []
    for node in cst.of_kind("CaseStatement"):
        fields = node.get("fields") or {}
        has_default = fields.get("has_default")
        is_unique = fields.get("is_unique") or fields.get("is_priority")
        if has_default is True or is_unique is True:
            continue
        first = node.get("first_token")
        if first is None:
            continue
        anchor = tokens[first]
        loc = byte_span_to_loc(anchor.get("start"), anchor.get("end"), line_starts)
        out.append({
            "rule_id": "case_has_default_branch",
            "severity": "warning",
            "message": "case statement must include a default item",
            "location": loc,
        })
    return out
