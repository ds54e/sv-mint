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
    nodes = list(cst.of_kind("ParameterDeclaration")) + list(
        cst.of_kind("LocalParameterDeclaration")
    )
    for node in nodes:
        fields = node.get("fields") or {}
        ty = _field_id(fields, "type")
        if ty is None or _is_implicit(cst, ty):
            tok = node.get("first_token")
            if tok is not None:
                violation = _violation(tokens, tok, line_starts)
                if violation:
                    out.append(violation)
    return out

def _field_id(fields, key):
    val = fields.get(key)
    if val is None:
        return None
    if isinstance(val, int):
        return val
    if isinstance(val, float):
        return int(val)
    return None

def _is_implicit(cst, node_id):
    node = cst.nodes_by_id.get(node_id)
    if not node:
        return True
    kind_id = node.get("kind", -1)
    if kind_id < 0 or kind_id >= len(cst.kinds):
        return True
    if "ImplicitDataType" in cst.kinds[kind_id]:
        children = node.get("children") or cst.children.get(node.get("id"), [])
        for cid in children:
            child = cst.nodes_by_id.get(cid)
            if child and "Signing" in _kind_name(cst, child):
                return False
        return True
    return False

def _violation(tokens, tok_idx, line_starts):
    tok = tokens[tok_idx] if tok_idx is not None and tok_idx < len(tokens) else None
    if not tok:
        return None
    start = tok.get("start")
    end = tok.get("end")
    if start is None or end is None:
        return None
    loc = byte_span_to_loc(start, end, line_starts)
    return {
        "rule_id": "parameter_has_type",
        "severity": "warning",
        "message": "parameter must declare an explicit data type",
        "location": loc,
    }

def _kind_name(cst, node):
    kind_id = node.get("kind", -1)
    if kind_id < 0 or kind_id >= len(cst.kinds):
        return ""
    return cst.kinds[kind_id]
