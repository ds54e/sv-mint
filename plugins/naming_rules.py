
import re

LOWER_SNAKE = re.compile(r"^[a-z][a-z0-9_]*$")
DIGIT_SUFFIX = re.compile(r"_[0-9]+$")


def check(req):
    if req.get("stage") != "ast":
        return []
    payload = req.get("payload") or {}
    decls = payload.get("decls") or []
    symbols = payload.get("symbols") or []
    ports = payload.get("ports") or []
    out = []
    for decl in decls:
        if decl.get("kind") != "module":
            continue
        name = decl.get("name") or ""
        loc = decl.get("loc")
        out.extend(validate_name(name, loc, "naming.module_case"))
    name_set = {sym.get("name") for sym in symbols}
    for sym in symbols:
        if sym.get("class") not in ("net", "var"):
            continue
        name = sym.get("name") or ""
        loc = sym.get("loc")
        out.extend(validate_name(name, loc, "naming.signal_case"))
        out.extend(check_suffixes(name, loc))
        out.extend(check_clock_reset(name, loc))
    for port in ports:
        name = port.get("name") or ""
        loc = port.get("loc")
        out.extend(validate_name(name, loc, "naming.port_case"))
        out.extend(check_suffixes(name, loc))
        out.extend(check_clock_reset(name, loc))
    out.extend(check_clock_reset_order(ports))
    out.extend(check_differential_pairs(ports))
    out.extend(check_port_direction_suffixes(ports))
    out.extend(check_pipeline_suffixes(name_set))
    out.extend(check_parameter_naming(decls))
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


def check_clock_reset_order(ports):
    issues = []
    clk_seen = False
    rst_phase = False
    for port in ports:
        name = port.get("name") or ""
        loc = port.get("loc")
        direction = port.get("direction") or ""
        if name.startswith("clk"):
            if clk_seen and rst_phase and loc:
                issues.append({
                    "rule_id": "naming.clk_order",
                    "severity": "warning",
                    "message": "clk ports should appear before resets and other ports",
                    "location": loc,
                })
            clk_seen = True
        elif name.startswith("rst"):
            if not clk_seen and loc:
                issues.append({
                    "rule_id": "naming.rst_before_clk",
                    "severity": "warning",
                    "message": "rst ports should follow clk ports",
                    "location": loc,
                })
            rst_phase = True
    return issues


def check_differential_pairs(ports):
    issues = []
    modules = {}
    for port in ports:
        modules.setdefault(port.get("module"), []).append(port)
    for module, plist in modules.items():
        names = {p.get("name") or "": p for p in plist}
        for name, port in names.items():
            if name.endswith("_p"):
                twin = name[:-2] + "_n"
                if twin not in names:
                    issues.append({
                        "rule_id": "naming.differential_pair",
                        "severity": "warning",
                        "message": f"differential pair missing counterpart for {name}",
                        "location": port.get("loc") or {"line": 1, "col": 1, "end_line": 1, "end_col": 1},
                    })
    return issues


def check_pipeline_suffixes(names):
    issues = []
    for name in names:
        if name.endswith("_q"):
            continue
        if name.endswith("_q0"):
            continue
        if re_match := _pipeline_match(name):
            base, stage = re_match
            prev = base + ("_q" if stage == 2 else f"_q{stage-1}")
            if prev not in names:
                issues.append({
                    "rule_id": "naming.pipeline_sequence",
                    "severity": "warning",
                    "message": f"pipeline stage {name} missing previous stage {prev}",
                    "location": {"line": 1, "col": 1, "end_line": 1, "end_col": 1},
                })
    return issues


def _pipeline_match(name):
    import re

    m = re.match(r"(.+)_q(\d+)$", name)
    if not m:
        return None
    stage = int(m.group(2))
    if stage < 2:
        return None
    return m.group(1), stage


def check_parameter_naming(decls):
    issues = []
    for decl in decls:
        if decl.get("kind") != "param":
            continue
        name = decl.get("name") or ""
        if not name or not name[0].isupper():
            issues.append({
                "rule_id": "naming.parameter_upper",
                "severity": "warning",
                "message": f"parameter {name} should use UpperCamelCase",
                "location": decl.get("loc") or {"line": 1, "col": 1, "end_line": 1, "end_col": 1},
            })
    return issues


def check_port_direction_suffixes(ports):
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
        loc = port.get("loc") or {"line": 1, "col": 1, "end_line": 1, "end_col": 1}
        if any(name.endswith(sfx) for sfx in allowed):
            continue
        exp = " or ".join(allowed)
        issues.append({
            "rule_id": "naming.port_suffix",
            "severity": "warning",
            "message": f"{name} must end with {exp}",
            "location": loc,
        })
    return issues
