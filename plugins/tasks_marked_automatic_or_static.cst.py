from lib.cst_inline import Cst, byte_span_to_loc

def check(req):
    if req.get("stage") != "cst":
        return []
    payload = req.get("payload") or {}
    ir = payload.get("cst_ir") or {}
    cst = Cst(ir)
    tokens = ir.get("tokens") or []
    line_starts = ir.get("line_starts") or [0]
    text = ir.get("source_text") or ir.get("pp_text") or ""
    out = []
    for node in cst.of_kind("TaskDeclaration"):
        if _has_scope_keyword(tokens, node, text):
            continue
        first = node.get("first_token")
        if first is None or first >= len(tokens):
            continue
        start = tokens[first].get("start")
        end = tokens[first].get("end")
        if start is None or end is None:
            continue
        loc = byte_span_to_loc(start, end, line_starts)
        out.append(
            {
                "rule_id": "tasks_marked_automatic_or_static",
                "severity": "warning",
                "message": "tasks in packages/modules/interfaces must declare automatic or static",
                "location": loc,
            }
        )
    return out

def _has_scope_keyword(tokens, node, text):
    first = node.get("first_token")
    last = node.get("last_token")
    if first is None or last is None:
        return False
    last = min(last, len(tokens) - 1)
    for tok in tokens[first : last + 1]:
        start = tok.get("start")
        end = tok.get("end")
        if start is None or end is None:
            continue
        word = text[start:end].lower()
        if word == "automatic" or word == "static":
            return True
        if word.startswith("task"):
            continue
        if word == ";":
            break
    return False
