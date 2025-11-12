import re

LOWER_SNAKE = re.compile(r"^[a-z][a-z0-9_]*$")
DIGIT_SUFFIX = re.compile(r"_[0-9]+$")


def check(req):
    if req.get("stage") != "ast":
        return []
    payload = req.get("payload") or {}
    decls = payload.get("decls") or []
    symbols = payload.get("symbols") or []
    out = []
    for decl in decls:
        if decl.get("kind") != "module":
            continue
        name = decl.get("name") or ""
        loc = decl.get("loc")
        out.extend(validate_name(name, loc, "naming.module_case"))
    for sym in symbols:
        if sym.get("class") not in ("net", "var"):
            continue
        name = sym.get("name") or ""
        loc = sym.get("loc")
        out.extend(validate_name(name, loc, "naming.signal_case"))
        out.extend(check_suffixes(name, loc))
        out.extend(check_clock_reset(name, loc))
    return out


def validate_name(name, loc, rule_id):
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
    elif DIGIT_SUFFIX.search(name):
        issues.append({
            "rule_id": "naming.no_numeric_suffix",
            "severity": "warning",
            "message": f"{name} must not end with _<number>",
            "location": loc,
        })
    return issues


def check_suffixes(name, loc):
    issues = []
    if loc is None:
        return issues
    if "_n_i" in name or "_n_o" in name or "_n_io" in name:
        issues.append({
            "rule_id": "naming.suffix_order",
            "severity": "warning",
            "message": f"{name} must combine suffixes without extra underscores (use '_ni', '_no', '_nio')",
            "location": loc,
        })
    return issues


def check_clock_reset(name, loc):
    issues = []
    if loc is None:
        return issues
    if "clk" in name and not name.startswith("clk"):
        issues.append({
            "rule_id": "naming.clk_prefix",
            "severity": "warning",
            "message": f"{name} must start with 'clk'",
            "location": loc,
        })
    if name.startswith("rst") and not (name.endswith("_n") or name.endswith("_ni") or name.endswith("_no") or name.endswith("_nio")):
        issues.append({
            "rule_id": "naming.rst_active_low",
            "severity": "warning",
            "message": f"{name} must end with '_n' for active-low resets",
            "location": loc,
        })
    return issues
