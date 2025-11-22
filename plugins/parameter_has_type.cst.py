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
    for node in cst.of_kind("ParameterDeclaration"):
        first = node.get("first_token")
        last = node.get("last_token")
        if first is None or last is None:
            continue
        first_word = _tok_text(tokens[first], text).lower()
        if first_word == "localparam":
            continue
        if _has_explicit_type(cst, node) or _has_type_tokens(tokens, first, last, text):
            continue
        start = tokens[first].get("start")
        end = tokens[first].get("end")
        if start is None or end is None:
            continue
        loc = byte_span_to_loc(start, end, line_starts)
        out.append({
            "rule_id": "parameter_has_type",
            "severity": "warning",
            "message": "parameter must declare an explicit data type",
            "location": loc,
        })
    return out


def _has_explicit_type(cst, node):
    stack = [node.get("id")]
    while stack:
        nid = stack.pop()
        n = cst.nodes_by_id.get(nid)
        if not n:
            continue
        kind_name = _kind_name(cst, n)
        if "ImplicitDataType" in kind_name:
            return False
        if kind_name.endswith("DataType") or kind_name.endswith("NetType"):
            return True
        for child in cst.children.get(nid, []):
            stack.append(child)
    return False


def _kind_name(cst, node):
    kind_id = node.get("kind")
    if kind_id is None:
        return ""
    if 0 <= kind_id < len(cst.kinds):
        return cst.kinds[kind_id]
    return ""


def _has_type_tokens(tokens, first, last, text):
    saw_parameter = False
    ident_count = 0
    for tok in tokens[first:last + 1]:
        word = _tok_text(tok, text)
        if not word:
            continue
        low = word.lower()
        if not saw_parameter:
            if low == "parameter":
                saw_parameter = True
            continue
        if word in ("=", ",", ";"):
            break
        if low == "type" or word.startswith("["):
            return True
        if word == "::":
            continue
        if word.isidentifier():
            ident_count += 1
            if ident_count >= 2:
                # type token + param identifier observed
                return True
    return False


def _tok_text(tok, text):
    start = tok.get("start")
    end = tok.get("end")
    if start is None or end is None:
        return ""
    return text[start:end].strip()
