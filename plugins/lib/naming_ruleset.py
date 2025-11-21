import re

CACHE_KEY = "__naming_ruleset"
LOWER_SNAKE = re.compile(r"^[a-z][a-z0-9_]*$")
DIGIT_SUFFIX = re.compile(r"_[0-9]+$")
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
    symbols = payload.get("symbols") or []
    ports = payload.get("ports") or []
    name_set = {sym.get("name") or "" for sym in symbols}
    collected = []
    collected.extend(_check_modules(decls))
    collected.extend(_check_symbols(symbols))
    collected.extend(_check_ports(ports))
    collected.extend(_check_clock_reset_order(ports))
    collected.extend(_check_differential_pairs(ports))
    collected.extend(_check_port_direction_suffixes(ports))
    collected.extend(_check_pipeline_suffixes(name_set))
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


def _check_symbols(symbols):
    out = []
    for sym in symbols:
        if sym.get("class") not in ("net", "var"):
            continue
        name = sym.get("name") or ""
        loc = sym.get("loc")
        out.extend(_check_suffixes(name, loc))
        out.extend(_check_clock_reset(name, loc))
    return out


def _check_ports(ports):
    out = []
    for port in ports:
        name = port.get("name") or ""
        loc = port.get("loc")
        out.extend(_validate_name(name, loc, "port_names_lower_snake"))
        out.extend(_check_suffixes(name, loc))
        out.extend(_check_clock_reset(name, loc))
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
    elif DIGIT_SUFFIX.search(name):
        issues.append({
            "rule_id": "naming_no_numeric_suffix",
            "severity": "warning",
            "message": f"{name} must not end with _<number>",
            "location": loc,
        })
    return issues


def _check_suffixes(name, loc):
    issues = []
    if loc is None:
        return issues
    if "_n_i" in name or "_n_o" in name or "_n_io" in name:
        issues.append({
            "rule_id": "naming_suffix_order",
            "severity": "warning",
            "message": f"{name} must combine suffixes without extra underscores (use '_ni', '_no', '_nio')",
            "location": loc,
        })
    return issues


def _check_clock_reset(name, loc):
    issues = []
    if loc is None:
        return issues
    if "clk" in name and not name.startswith("clk"):
        issues.append({
            "rule_id": "naming_clock_prefix",
            "severity": "warning",
            "message": f"{name} must start with 'clk'",
            "location": loc,
        })
    if name.startswith("rst") and not (name.endswith("_n") or name.endswith("_ni") or name.endswith("_no") or name.endswith("_nio")):
        issues.append({
            "rule_id": "naming_reset_active_low_suffix",
            "severity": "warning",
            "message": f"{name} must end with '_n' for active-low resets",
            "location": loc,
        })
    return issues


def _check_clock_reset_order(ports):
    issues = []
    clk_seen = False
    rst_phase = False
    for port in ports:
        name = port.get("name") or ""
        loc = port.get("loc")
        if name.startswith("clk"):
            if clk_seen and rst_phase and loc:
                issues.append({
                    "rule_id": "naming_clock_port_order",
                    "severity": "warning",
                    "message": "clk ports should appear before resets and other ports",
                    "location": loc,
                })
            clk_seen = True
        elif name.startswith("rst"):
            if not clk_seen and loc:
                issues.append({
                    "rule_id": "naming_reset_after_clock",
                    "severity": "warning",
                    "message": "rst ports should follow clk ports",
                    "location": loc,
                })
            rst_phase = True
    return issues


def _check_differential_pairs(ports):
    issues = []
    modules = {}
    for port in ports:
        modules.setdefault(port.get("module"), []).append(port)
    for plist in modules.values():
        names = {p.get("name") or "": p for p in plist}
        for name, port in names.items():
            if name.endswith("_p"):
                twin = name[:-2] + "_n"
                if twin not in names:
                    issues.append({
                        "rule_id": "naming_differential_pair",
                        "severity": "warning",
                        "message": f"differential pair missing counterpart for {name}",
                        "location": port.get("loc") or _default_loc(),
                    })
    return issues


def _check_pipeline_suffixes(names):
    issues = []
    for name in names:
        if name.endswith("_q") or name.endswith("_q0"):
            continue
        re_match = _pipeline_match(name)
        if re_match:
            base, stage = re_match
            prev = base + ("_q" if stage == 2 else f"_q{stage-1}")
            if prev not in names:
                issues.append({
                    "rule_id": "naming_pipeline_sequence",
                    "severity": "warning",
                    "message": f"pipeline stage {name} missing previous stage {prev}",
                    "location": _default_loc(),
                })
    return issues


def _pipeline_match(name):
    m = re.match(r"(.+)_q(\d+)$", name)
    if not m:
        return None
    stage = int(m.group(2))
    if stage < 2:
        return None
    return m.group(1), stage


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


def _check_port_direction_suffixes(ports):
    issues = []
    suffixes = {
        "input": ("_i", "_ni"),
        "output": ("_o", "_no"),
        "inout": ("_io", "_nio"),
    }
    for port in ports:
        direction = (port.get("direction") or "").lower()
        allowed = suffixes.get(direction)
        if not allowed:
            continue
        name = port.get("name") or ""
        if not name:
            continue
        loc = port.get("loc") or _default_loc()
        if any(name.endswith(sfx) for sfx in allowed):
            continue
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
