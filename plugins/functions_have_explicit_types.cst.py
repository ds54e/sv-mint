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
        ret_id = _field_id(fields, "return_type")
        if ret_id is None or _is_implicit(cst, ret_id):
            tok = _function_name_token(cst, node)
            if tok is not None:
                violation = _violation(tokens, tok, line_starts, "function must declare an explicit return type")
                if violation:
                    out.append(violation)
            continue
        for port in fields.get("ports") or []:
            if _is_implicit_port(cst, port):
                tok = port.get("name_token")
                if tok is None:
                    tok = _function_name_token(cst, node)
                if tok is not None:
                    violation = _violation(tokens, tok, line_starts, "function arguments must declare explicit data types")
                    if violation:
                        out.append(violation)
                break
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
    if cst.kinds[kind_id] == "ImplicitDataType":
        children = node.get("children") or cst.children.get(node.get("id"), [])
        for cid in children:
            child = cst.nodes_by_id.get(cid)
            if child and "Signing" in _kind_name(cst, child):
                return False
        return True
    return False


def _is_implicit_port(cst, port):
    ty = port.get("type")
    if ty is None:
        return True
    node = cst.nodes_by_id.get(int(ty))
    if not node:
        return True
    if _kind_name(cst, node) == "ImplicitDataType":
        return True
    name_tok = port.get("name_token")
    if name_tok is not None and _kind_name(cst, node) == "DataType":
        if node.get("first_token") == int(name_tok) and node.get("last_token") == int(name_tok):
            return True
    return False


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


def _kind_name(cst, node):
    kind_id = node.get("kind", -1)
    if kind_id < 0 or kind_id >= len(cst.kinds):
        return ""
    return cst.kinds[kind_id]


def _violation(tokens, tok_idx, line_starts, msg):
    tok = tokens[tok_idx] if tok_idx is not None and tok_idx < len(tokens) else None
    if not tok:
        return None
    start = tok.get("start")
    end = tok.get("end")
    if start is None or end is None:
        return None
    loc = byte_span_to_loc(start, end, line_starts)
    return {
        "rule_id": "functions_have_explicit_types",
        "severity": "warning",
        "message": msg,
        "location": loc,
    }
