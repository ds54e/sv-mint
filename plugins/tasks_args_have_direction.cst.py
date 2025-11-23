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
        if port.get("dir") is not None:
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
        loc = _token_loc(tokens, tok, line_starts)
        if loc:
            out.append(
                {
                    "rule_id": "tasks_args_have_direction",
                    "severity": "warning",
                    "message": "task arguments must specify direction (input/output/inout/ref)",
                    "location": loc,
                }
            )
            reported.add(task_id)
    return out

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
