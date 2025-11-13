import re

FUNCTION_RE = re.compile(r"\bfunction\b", re.IGNORECASE)
RANDOMIZE_RE = re.compile(r"\brandomize\s*\(", re.IGNORECASE)
LOG_CALL_RE = re.compile(r"\buvm_(info|error|fatal)\s*\(", re.IGNORECASE)
UVM_WARNING_RE = re.compile(r"\buvm_warning\s*\(", re.IGNORECASE)
UVM_REPORT_RE = re.compile(r"\buvm_report_[A-Za-z_]+\s*\(", re.IGNORECASE)
DISPLAY_RE = re.compile(r"\$display\b")
UVM_NONE_FULL_RE = re.compile(r"\bUVM_(NONE|FULL)\b")
IMPORT_RE = re.compile(r'import\s+"DPI"[^;]*?\b([A-Za-z_]\w*)\s*\(', re.IGNORECASE)
EXPORT_RE = re.compile(r'export\s+"DPI"\s+(?:function|task)?\s*([A-Za-z_]\w*)\s*=', re.IGNORECASE)
DEFINE_RE = re.compile(r"`define\s+([A-Za-z_]\w*)")
UNDEF_RE = re.compile(r"`undef\s+([A-Za-z_]\w*)")
WAIT_FORK_RE = re.compile(r"\bwait\s+fork\b", re.IGNORECASE)
WAIT_STMT_RE = re.compile(r"\bwait\s*\(", re.IGNORECASE)


def check(req):
    if req.get("stage") != "raw_text":
        return []
    payload = req.get("payload") or {}
    text = payload.get("text") or ""
    path = req.get("path") or ""
    out = []
    out.extend(_check_function_scope(text))
    out.extend(_check_randomize(text))
    out.extend(_check_logging(text))
    out.extend(_check_dpi(text))
    out.extend(_check_macros(text, path))
    out.extend(_check_wait_usage(text))
    return out


def _loc(text, index):
    line = text.count("\n", 0, index) + 1
    prev = text.rfind("\n", 0, index)
    col = index + 1 if prev < 0 else index - prev
    return {"line": line, "col": col, "end_line": line, "end_col": col + 1}


def _check_function_scope(text):
    scopes = []
    class_depth = 0
    out = []
    offset = 0
    for chunk in text.splitlines(True):
        line = chunk[:-1] if chunk.endswith("\n") else chunk
        stripped = line.strip()
        lower = stripped.lower()
        if lower.startswith("endclass"):
            class_depth = max(0, class_depth - 1)
        if lower.startswith("endpackage"):
            scopes = _pop_scope(scopes, "package")
        elif lower.startswith("endmodule"):
            scopes = _pop_scope(scopes, "module")
        elif lower.startswith("endinterface"):
            scopes = _pop_scope(scopes, "interface")
        elif lower.startswith("endprogram"):
            scopes = _pop_scope(scopes, "program")
        if lower.startswith("class "):
            class_depth += 1
        if lower.startswith("package "):
            scopes.append("package")
        elif lower.startswith("module "):
            scopes.append("module")
        elif lower.startswith("interface "):
            scopes.append("interface")
        elif lower.startswith("program "):
            scopes.append("program")
        if scopes and class_depth == 0:
            for match in FUNCTION_RE.finditer(line):
                prefix = line[:match.start()].strip().lower()
                if prefix.endswith("end"):
                    continue
                segment = line[match.start():].lower()
                if " automatic" in segment or " static" in segment:
                    continue
                out.append({
                    "rule_id": "style.function_scope",
                    "severity": "warning",
                    "message": "functions in packages/modules/interfaces must declare automatic or static",
                    "location": _loc(text, offset + match.start()),
                })
        offset += len(chunk)
    return out


def _pop_scope(scopes, kind):
    if not scopes:
        return scopes
    if scopes[-1] == kind:
        return scopes[:-1]
    return scopes


def _check_randomize(text):
    out = []
    for match in RANDOMIZE_RE.finditer(text):
        out.append({
            "rule_id": "rand.dv_macro_required",
            "severity": "warning",
            "message": "use DV_CHECK_* randomization macros instead of direct randomize()",
            "location": _loc(text, match.start()),
        })
    return out


def _check_logging(text):
    out = []
    for match in LOG_CALL_RE.finditer(text):
        arg = _first_arg(text, match.end())
        norm = arg.lstrip("`").strip()
        if norm not in ("gfn", "gtn"):
            out.append({
                "rule_id": "log.uvm_arg_macro",
                "severity": "warning",
                "message": "uvm report macros must use gfn/gtn as the message tag",
                "location": _loc(text, match.start()),
            })
    for pattern, rule_id, message in (
        (UVM_WARNING_RE, "log.no_uvm_warning", "use uvm_error or uvm_fatal instead of uvm_warning"),
        (UVM_REPORT_RE, "log.no_uvm_report_api", "use uvm_{info,error,fatal} macros instead of uvm_report_*"),
        (DISPLAY_RE, "log.no_display", "use uvm_* macros instead of $display"),
    ):
        for match in pattern.finditer(text):
            out.append({
                "rule_id": rule_id,
                "severity": "warning",
                "message": message,
                "location": _loc(text, match.start()),
            })
    for match in UVM_NONE_FULL_RE.finditer(text):
        out.append({
            "rule_id": "log.no_none_full",
            "severity": "warning",
            "message": "avoid UVM_NONE and UVM_FULL verbosity levels",
            "location": _loc(text, match.start()),
        })
    return out


def _first_arg(text, index):
    depth = 0
    arg_start = None
    i = index
    while i < len(text):
        ch = text[i]
        if ch == "(":
            depth += 1
            if depth == 1:
                arg_start = i + 1
        elif ch == ")":
            if depth == 1:
                if arg_start is None:
                    return ""
                return text[arg_start:i].strip()
            depth -= 1
        elif ch == "," and depth == 1:
            if arg_start is None:
                return ""
            return text[arg_start:i].strip()
        i += 1
    return ""


def _check_dpi(text):
    out = []
    for match in IMPORT_RE.finditer(text):
        name = match.group(1)
        if not name.startswith("c_dpi_"):
            out.append({
                "rule_id": "dpi.import_prefix",
                "severity": "warning",
                "message": "imported DPI functions must start with c_dpi_",
                "location": _loc(text, match.start(1)),
            })
    for match in EXPORT_RE.finditer(text):
        name = match.group(1)
        if not name.startswith("sv_dpi_"):
            out.append({
                "rule_id": "dpi.export_prefix",
                "severity": "warning",
                "message": "exported DPI handles must start with sv_dpi_",
                "location": _loc(text, match.start(1)),
            })
    return out


def _check_macros(text, path):
    out = []
    defs = []
    for match in DEFINE_RE.finditer(text):
        defs.append((match.group(1), match.start(1)))
    undefs = {match.group(1) for match in UNDEF_RE.finditer(text)}
    if not path.endswith("_macros.svh"):
        for name, index in defs:
            if name not in undefs:
                out.append({
                    "rule_id": "macro.missing_undef",
                    "severity": "warning",
                    "message": f"`define {name} must be undefined at end of file",
                    "location": _loc(text, index),
                })
    return out


def _check_wait_usage(text):
    out = []
    for match in WAIT_FORK_RE.finditer(text):
        out.append({
            "rule_id": "flow.wait_fork_isolation",
            "severity": "warning",
            "message": "wait fork must be wrapped in an isolation fork (prefer DV_SPINWAIT)",
            "location": _loc(text, match.start()),
        })
    for match in WAIT_STMT_RE.finditer(text):
        out.append({
            "rule_id": "flow.wait_macro_required",
            "severity": "warning",
            "message": "use DV_WAIT macro instead of raw wait statements",
            "location": _loc(text, match.start()),
        })
    return out
