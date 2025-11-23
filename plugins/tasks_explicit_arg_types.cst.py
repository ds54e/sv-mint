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
    reported = set()
    for node in cst.of_kind("TfPortItem"):
        port = (node.get("fields") or {}).get("port") or {}
        if not _is_implicit_port(cst, port):
            continue
        task_id = _enclosing_task_id(cst, node)
        if task_id is None or task_id in reported:
            continue
        task_node = cst.nodes_by_id.get(task_id)
        tok = port.get("name_token")
        if tok is None and task_node is not None:
            tok = _task_name_token(cst, task_node)
        if tok is None:
            continue
        violation = _violation(tokens, tok, line_starts)
        if violation:
            out.append(violation)
            reported.add(task_id)
    return out

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

def _task_name_token(cst, node):
    stack = list(node.get("children") or cst.children.get(node.get("id"), []))
    while stack:
        cid = stack.pop()
        child = cst.nodes_by_id.get(cid)
        if not child:
            continue
        if _kind_name(cst, child) == "TaskIdentifier":
            return child.get("first_token")
        stack.extend(child.get("children") or cst.children.get(cid, []))
    return None

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
        "rule_id": "tasks_explicit_arg_types",
        "severity": "warning",
        "message": "task arguments must declare explicit data types",
        "location": loc,
    }

def _kind_name(cst, node):
    kind_id = node.get("kind", -1)
    if kind_id < 0 or kind_id >= len(cst.kinds):
        return ""
    return cst.kinds[kind_id]

def _enclosing_task_id(cst, node):
    current = node
    while current:
        if _kind_name(cst, current) == "TaskDeclaration":
            return current.get("id")
        parent_id = current.get("parent")
        if parent_id is None:
            break
        current = cst.nodes_by_id.get(parent_id)
    return None
