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
    for node in cst.of_kind("FunctionDeclaration"):
        first = node.get("first_token")
        last = node.get("last_token")
        if first is None or last is None:
            continue
        ret_token = _return_type_token(tokens, first, last, text)
        if ret_token is not None:
            loc = byte_span_to_loc(tokens[ret_token]["start"], tokens[ret_token]["end"], line_starts)
            out.append({
                "rule_id": "functions_have_explicit_types",
                "severity": "warning",
                "message": "function must declare an explicit return type",
                "location": loc,
            })
            continue
        arg_token = _implicit_arg_token(tokens, first, last, text)
        if arg_token is not None:
            loc = byte_span_to_loc(tokens[arg_token]["start"], tokens[arg_token]["end"], line_starts)
            out.append({
                "rule_id": "functions_have_explicit_types",
                "severity": "warning",
                "message": "function arguments must declare explicit data types",
                "location": loc,
            })
    return out


def _return_type_token(tokens, first, last, text):
    saw_function = False
    type_seen = False
    ident_seen = 0
    for i in range(first, last + 1):
        word = _tok_text(tokens[i], text)
        if not word:
            continue
        low = word.lower()
        if not saw_function:
            if low.startswith("function"):
                saw_function = True
            continue
        if word == "(":
            break
        if low in ("automatic", "static", "virtual", "pure", "extern", "local"):
            continue
        if low == "void":
            return None
        if low in _type_keywords() or word.startswith("["):
            type_seen = True
            continue
        if word == "::":
            type_seen = True
            continue
        if word in ("=", ",", ";"):
            break
        if word.isidentifier():
            ident_seen += 1
            if type_seen or ident_seen >= 2:
                return None
            return i
    return None


def _implicit_arg_token(tokens, first, last, text):
    lparen = _find_token(tokens, first, last, "(")
    if lparen is None:
        return None
    rparen = _match_paren(tokens, lparen, last, text)
    if rparen is None:
        return None
    start = lparen + 1
    while start < rparen:
        end = _next_comma(tokens, start, rparen, text)
        tok = _arg_missing_type(tokens, start, end, text)
        if tok is not None:
            return tok
        start = end + 1
    return None


def _arg_missing_type(tokens, start, end, text):
    type_seen = False
    ident_seen = 0
    for i in range(start, end):
        word = _tok_text(tokens[i], text)
        if not word:
            continue
        low = word.lower()
        if word == ")":
            break
        if low in ("input", "output", "inout", "ref", "const", "var"):
            continue
        if low in _type_keywords() or word.startswith("[") or word == "::":
            type_seen = True
            continue
        if word == "=":
            break
        if word.isidentifier():
            ident_seen += 1
            if type_seen:
                continue
            if ident_seen >= 2:
                type_seen = True
                continue
            return i
    if type_seen:
        return None
    return start if start < end else None


def _find_token(tokens, first, last, needle):
    for i in range(first, last + 1):
        if _tok_text(tokens[i], None, needle):
            return i
    return None


def _match_paren(tokens, lparen, last, text):
    depth = 0
    for i in range(lparen, last + 1):
        word = _tok_text(tokens[i], text)
        if word == "(":
            depth += 1
        elif word == ")":
            depth -= 1
            if depth == 0:
                return i
    return None


def _next_comma(tokens, start, end, text):
    depth = 0
    for i in range(start, end):
        word = _tok_text(tokens[i], text)
        if word == "(":
            depth += 1
        elif word == ")":
            if depth == 0:
                return i
            depth -= 1
        elif word == "," and depth == 0:
            return i
    return end


def _type_keywords():
    return {
        "bit", "logic", "reg", "int", "integer", "longint", "shortint", "byte",
        "time", "realtime", "real", "shortreal", "string", "wire", "signed", "unsigned",
    }


def _tok_text(tok, text, needle=None):
    start = tok.get("start")
    end = tok.get("end")
    if start is None or end is None:
        return ""
    if needle is None:
        return (text or "")[start:end].strip()
    return (text or "")[start:end].strip() == needle
