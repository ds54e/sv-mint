import re

ALL_CAPS = re.compile(r"^[A-Z][A-Z0-9_]*$")

def check(req):
    if req.get("stage") != "ast":
        return []
    payload = req.get("payload") or {}
    decls = payload.get("decls") or []
    out = []
    for d in decls:
        if d.get("kind") != "localparam":
            continue
        name = d.get("name") or ""
        if ALL_CAPS.match(name):
            continue
        out.append(
            {
                "rule_id": "localparam_names_uppercase",
                "severity": "warning",
                "message": f"localparam {name} should use ALL_CAPS",
                "location": d.get(
                    "loc", {"line": 1, "col": 1, "end_line": 1, "end_col": 1}
                ),
            }
        )
    return out
