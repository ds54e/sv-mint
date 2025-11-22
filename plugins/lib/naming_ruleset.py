import re

CACHE_KEY = "__naming_ruleset"
LOWER_SNAKE = re.compile(r"^[a-z][a-z0-9_]*$")
ALL_CAPS = re.compile(r"^[A-Z][A-Z0-9_]*$")
UPPER_CAMEL = re.compile(r"^[A-Z][A-Za-z0-9]*$")


def violations_for(req, rule_id):
    table = evaluate(req)
    return list(table.get(rule_id) or [])


def evaluate(req):
    cached = req.get(CACHE_KEY)
    if cached is not None:
        return cached
    if req.get("stage") != "ast":
        req[CACHE_KEY] = {}
        return req[CACHE_KEY]
    payload = req.get("payload") or {}
    decls = payload.get("decls") or []
    ports = payload.get("ports") or []
    collected = []
    collected.extend(_check_modules(decls))
    collected.extend(_check_ports(ports))
    collected.extend(_check_parameter_naming(decls))
    table = {}
    for item in collected:
        rule_id = item.get("rule_id")
        if not rule_id:
            continue
        table.setdefault(rule_id, []).append(item)
    req[CACHE_KEY] = table
    return table


def _check_modules(decls):
    out = []
    for decl in decls:
        if decl.get("kind") != "module":
            continue
        name = decl.get("name") or ""
        loc = decl.get("loc")
        out.extend(_validate_name(name, loc, "module_names_lower_snake"))
    return out


def _check_ports(ports):
    out = []
    for port in ports:
        name = port.get("name") or ""
        loc = port.get("loc")
        out.extend(_validate_name(name, loc, "port_names_lower_snake"))
        out.extend(_check_port_direction_suffixes(port))
    return out


def _validate_name(name, loc, rule_id):
    issues = []
    if not name or loc is None:
        return issues
    if not LOWER_SNAKE.match(name):
        issues.append({
            "rule_id": rule_id,
            "severity": "warning",
            "message": f"{name} must use lower_snake_case",
            "location": loc,
        })
    return issues


def _check_parameter_naming(decls):
    issues = []
    for decl in decls:
        if decl.get("kind") != "param":
            continue
        name = decl.get("name") or ""
        if not name:
            continue
        if not (UPPER_CAMEL.match(name) or ALL_CAPS.match(name)):
            issues.append({
                "rule_id": "parameter_names_uppercase",
                "severity": "warning",
                "message": f"parameter {name} should use UpperCamelCase or ALL_CAPS",
                "location": decl.get("loc") or _default_loc(),
            })
    return issues


def _check_port_direction_suffixes(port):
    issues = []
    suffixes = {
        "input": ("_i", "_ni"),
        "output": ("_o", "_no"),
        "inout": ("_io", "_nio"),
    }
    direction = (port.get("direction") or "").lower()
    allowed = suffixes.get(direction)
    if not allowed:
        return issues
    name = port.get("name") or ""
    if not name:
        return issues
    loc = port.get("loc") or _default_loc()
    if any(name.endswith(sfx) for sfx in allowed):
        return issues
    exp = " or ".join(allowed)
    issues.append({
        "rule_id": "port_names_have_direction_suffix",
        "severity": "warning",
        "message": f"{name} must end with {exp}",
        "location": loc,
    })
    return issues


def _default_loc():
    return {"line": 1, "col": 1, "end_line": 1, "end_col": 1}
