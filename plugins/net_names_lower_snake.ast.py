import re


LOWER_SNAKE_DOLLAR = re.compile(r"^[a-z][a-z0-9_$]*$")


def check(req):
    if req.get("stage") != "ast":
        return []
    payload = req.get("payload") or {}
    symbols = payload.get("symbols") or []
    out = []
    for s in symbols:
        if s.get("class") != "net":
            continue
        name = s.get("name") or ""
        if not LOWER_SNAKE_DOLLAR.match(name):
            out.append({
                "rule_id": "net_names_lower_snake",
                "severity": "warning",
                "message": f"net names should use lower_snake_case (letters, digits, _, $ allowed): {name}",
                "location": s.get("loc", {"line": 1, "col": 1, "end_line": 1, "end_col": 1}),
            })
    return out
