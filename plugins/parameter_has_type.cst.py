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
        if _has_type(tokens, first, last, text):
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


def _has_type(tokens, first, last, text):
    saw_parameter = False
    for tok in tokens[first:last + 1]:
        word = _tok_text(tok, text).lower()
        if not word:
            continue
        if not saw_parameter:
            if word == "parameter":
                saw_parameter = True
            continue
        if word in ("type", "bit", "logic", "reg", "int", "integer", "longint", "shortint",
                    "byte", "time", "realtime", "real", "shortreal", "string", "wire",
                    "signed", "unsigned") or word.startswith("["):
            return True
        if word.isidentifier():
            # hit the identifier without seeing a type/range
            return False
    return False


def _tok_text(tok, text):
    start = tok.get("start")
    end = tok.get("end")
    if start is None or end is None:
        return ""
    return text[start:end].strip()
