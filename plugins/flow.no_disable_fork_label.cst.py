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
    for node in cst.of_kind("DisableStatement"):
        first = node.get("first_token")
        last = node.get("last_token")
        if first is None or last is None:
            continue
        # Check if this is "disable fork" (no label) or "disable <label>"
        label = _disable_label(tokens, first, last, text)
        if label is None:
            # disable without label (e.g., disable fork;) is fine
            continue
        start = tokens[first].get("start")
        end = tokens[first].get("end")
        if start is None or end is None:
            continue
        loc = byte_span_to_loc(start, end, line_starts)
        out.append({
            "rule_id": "flow.no_disable_fork_label",
            "severity": "warning",
            "message": "disable fork label is not portable; prefer disable fork",
            "location": loc,
        })
    return out


def _disable_label(tokens, first, last, text):
    # Expect pattern: disable <id_or_fork> ... ;
    for tok in tokens[first:last + 1]:
        start = tok.get("start")
        end = tok.get("end")
        if start is None or end is None:
            continue
        word = text[start:end].strip()
        if word.lower() == "disable":
            continue
        if word == ";":
            break
        # if word is "fork", treat as no label
        if word.lower() == "fork":
            return None
        # first non-disable, non-semicolon token is treated as label/target
        return word
    return None
