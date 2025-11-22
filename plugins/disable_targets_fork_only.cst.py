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
    for node in cst.of_kind("DisableStatement"):
        first = node.get("first_token")
        last = node.get("last_token")
        if first is None or last is None:
            continue
        label = _disable_label(tokens, first, last, text)
        if label is None:
            continue
        start = tokens[first].get("start")
        end = tokens[first].get("end")
        if start is None or end is None:
            continue
        loc = byte_span_to_loc(start, end, line_starts)
        out.append(
            {
                "rule_id": "disable_targets_fork_only",
                "severity": "warning",
                "message": "disable fork label is not portable; prefer disable fork",
                "location": loc,
            }
        )
    return out

def _disable_label(tokens, first, last, text):
    for tok in tokens[first : last + 1]:
        start = tok.get("start")
        end = tok.get("end")
        if start is None or end is None:
            continue
        word = text[start:end].strip()
        if not word:
            continue
        lower = word.lower()
        if lower == "disable":
            continue
        if word == ";":
            break
        if lower == "fork":
            return None
        return word
    return None
