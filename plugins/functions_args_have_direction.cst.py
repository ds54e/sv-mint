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
    for node in cst.of_kind("FunctionDeclaration"):
        fields = node.get("fields") or {}
        for port in fields.get("ports") or []:
            if port.get("dir") is None:
                tok = port.get("name_token")
                if tok is None:
                    tok = _function_name_token(cst, node)
                if tok is None:
                    continue
                loc = _token_loc(tokens, tok, line_starts)
                if loc:
                    out.append(
                        {
                            "rule_id": "functions_args_have_direction",
                            "severity": "warning",
                            "message": "function arguments must specify direction (input/output/inout/ref)",
                            "location": loc,
                        }
                    )
                break
    return out

def _function_name_token(cst, node):
    stack = list(node.get("children") or cst.children.get(node.get("id"), []))
    while stack:
        cid = stack.pop()
        child = cst.nodes_by_id.get(cid)
        if not child:
            continue
        if _kind_name(cst, child) == "FunctionIdentifier":
            return child.get("first_token")
        stack.extend(child.get("children") or cst.children.get(cid, []))
    return None

def _token_loc(tokens, tok_idx, line_starts):
    tok = tokens[tok_idx] if tok_idx is not None and tok_idx < len(tokens) else None
    if not tok:
        return None
    start = tok.get("start")
    end = tok.get("end")
    if start is None or end is None:
        return None
    return byte_span_to_loc(start, end, line_starts)

def _kind_name(cst, node):
    kind_id = node.get("kind", -1)
    if kind_id < 0 or kind_id >= len(cst.kinds):
        return ""
    return cst.kinds[kind_id]
