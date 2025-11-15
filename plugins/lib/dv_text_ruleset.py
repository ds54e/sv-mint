import re

from lib.dv_helpers import loc, raw_text_inputs

FUNCTION_RE = re.compile(r"\bfunction\b", re.IGNORECASE)
RANDOMIZE_RE = re.compile(r"\brandomize\s*\(", re.IGNORECASE)
RANDOMIZE_WITH_RE = re.compile(r"\brandomize\s*\([^;]*?\)\s*with\s*\{", re.IGNORECASE)
LOG_CALL_RE = re.compile(r"\buvm_(info|error|fatal)\s*\(", re.IGNORECASE)
UVM_WARNING_RE = re.compile(r"\buvm_warning\s*\(", re.IGNORECASE)
UVM_REPORT_RE = re.compile(r"\buvm_report_[A-Za-z_]+\s*\(", re.IGNORECASE)
DISPLAY_RE = re.compile(r"\$display\b")
UVM_NONE_FULL_RE = re.compile(r"\bUVM_(NONE|FULL)\b")
IMPORT_RE = re.compile(r'import\s+"DPI"[^;]*?\b([A-Za-z_]\w*)\s*\(', re.IGNORECASE)
EXPORT_RE = re.compile(r'export\s+"DPI"\s+(?:function|task)?\s*([A-Za-z_]\w*)\s*=', re.IGNORECASE)
DEFINE_RE = re.compile(r"`define\s+([A-Za-z_]\w*)")
UNDEF_RE = re.compile(r"`undef\s+([A-Za-z_]\w*)")
UVM_DO_RE = re.compile(r"`uvm_do", re.IGNORECASE)
IFNDEF_RE = re.compile(r"`ifndef\s+([A-Za-z_]\w*)")
STD_RANDOMIZE_RE = re.compile(r"\bstd::randomize\s*\(", re.IGNORECASE)
THIS_RANDOMIZE_RE = re.compile(r"\bthis\s*\.\s*randomize\s*\(", re.IGNORECASE)
IF_COMPARE_RE = re.compile(r"\bif\s*\([^;]*?(==|!=|<=|>=|<|>)", re.IGNORECASE)
UVM_REPORT_CALL_RE = re.compile(r"\buvm_(info|error|fatal)\s*\(", re.IGNORECASE)
MODULE_RE = re.compile(r"\bmodule\s+([A-Za-z_]\w*)", re.IGNORECASE)
SCOREBOARD_CLASS_RE = re.compile(r"class\s+([A-Za-z_]\w*scoreboard)\b", re.IGNORECASE)
DV_EOT_RE = re.compile(r"DV_EOT_PRINT_", re.IGNORECASE)
PROGRAM_RE = re.compile(r"\bprogram\b", re.IGNORECASE)
DV_MACRO_RE = re.compile(r"`define\s+(DV_[A-Za-z_]\w*)")
ALLOWED_VERBOSITY = {"UVM_LOW", "UVM_MEDIUM", "UVM_HIGH", "UVM_DEBUG"}
CACHE_KEY = "__dv_text_ruleset"


def violations_for(req, rule_id):
    table = evaluate(req)
    items = table.get(rule_id) or []
    return list(items)


def evaluate(req):
    cached = req.get(CACHE_KEY)
    if cached is not None:
        return cached
    inputs = raw_text_inputs(req)
    if not inputs:
        req[CACHE_KEY] = {}
        return req[CACHE_KEY]
    text, path = inputs
    collected = []
    collected.extend(_check_function_scope(text))
    collected.extend(_check_randomize(text))
    collected.extend(_check_logging(text))
    collected.extend(_check_dpi(text))
    collected.extend(_check_macros(text, path))
    collected.extend(_check_uvm_do(text))
    collected.extend(_check_scoreboard(text))
    collected.extend(_check_program(text))
    collected.extend(_check_comparison_macros(text))
    collected.extend(_check_module_macro_prefix(text))
    table = {}
    for item in collected:
        key = item.get("rule_id")
        if not key:
            continue
        table.setdefault(key, []).append(item)
    req[CACHE_KEY] = table
    return table


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
                    "location": loc(text, offset + match.start()),
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
    out.extend(_randomize_matches(text, RANDOMIZE_RE))
    out.extend(_randomize_matches(text, STD_RANDOMIZE_RE))
    out.extend(_randomize_matches(text, THIS_RANDOMIZE_RE))
    out.extend(_randomize_matches(
        text,
        RANDOMIZE_WITH_RE,
        rule_id="rand.dv_macro_with_required",
        message="use DV_CHECK_*_WITH macros when randomizing with constraints",
    ))
    return out


def _randomize_matches(text, pattern, rule_id="rand.dv_macro_required", message="use DV_CHECK_* randomization macros instead of direct randomize()"):
    out = []
    for match in pattern.finditer(text):
        prefix = text[max(0, match.start() - 40):match.start()]
        if "DV_CHECK" in prefix:
            continue
        out.append({
            "rule_id": rule_id,
            "severity": "warning",
            "message": message,
            "location": loc(text, match.start()),
        })
    return out


def _check_logging(text):
    out = []
    for match in LOG_CALL_RE.finditer(text):
        prefix = text[max(0, match.start() - 20):match.start()].lower()
        if "function" in prefix:
            continue
        args = _call_args(text, match.end())
        if not args:
            continue
        arg = args[0]
        norm = arg.lstrip("`").strip()
        if norm not in ("gfn", "gtn"):
            out.append({
                "rule_id": "log.uvm_arg_macro",
                "severity": "warning",
                "message": "uvm report macros must use gfn/gtn as the message tag",
                "location": loc(text, match.start()),
            })
        if len(args) >= 3:
            verb = args[2].strip()
            if verb not in ALLOWED_VERBOSITY:
                out.append({
                    "rule_id": "log.allowed_verbosity",
                    "severity": "warning",
                    "message": "uvm report macros must use UVM_LOW/MEDIUM/HIGH/DEBUG verbosity",
                    "location": loc(text, match.start()),
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
                "location": loc(text, match.start()),
            })
    for match in UVM_NONE_FULL_RE.finditer(text):
        out.append({
            "rule_id": "log.no_none_full",
            "severity": "warning",
            "message": "avoid UVM_NONE and UVM_FULL verbosity levels",
            "location": loc(text, match.start()),
        })
    return out


def _call_args(text, index):
    depth = 1
    start = index
    args = []
    i = index
    while i < len(text):
        ch = text[i]
        if ch == "(":
            depth += 1
        elif ch == ")":
            depth -= 1
            if depth == 0:
                if start is not None:
                    args.append(text[start:i].strip())
                break
        elif ch == "," and depth == 1:
            if start is not None:
                args.append(text[start:i].strip())
            start = i + 1
        i += 1
    return args


def _check_dpi(text):
    out = []
    for match in IMPORT_RE.finditer(text):
        name = match.group(1)
        if not name.startswith("c_dpi_"):
            out.append({
                "rule_id": "dpi.import_prefix",
                "severity": "warning",
                "message": "imported DPI functions must start with c_dpi_",
                "location": loc(text, match.start(1)),
            })
    for match in EXPORT_RE.finditer(text):
        name = match.group(1)
        if not name.startswith("sv_dpi_"):
            out.append({
                "rule_id": "dpi.export_prefix",
                "severity": "warning",
                "message": "exported DPI handles must start with sv_dpi_",
                "location": loc(text, match.start(1)),
            })
    return out


def _check_macros(text, path):
    out = []
    defs = []
    guard_positions = {}
    for match in IFNDEF_RE.finditer(text):
        guard_positions.setdefault(match.group(1), []).append(match.start())
    for match in DEFINE_RE.finditer(text):
        defs.append((match.group(1), match.start(1)))
    undefs = {match.group(1) for match in UNDEF_RE.finditer(text)}
    dv_macros = [match.group(1) for match in DV_MACRO_RE.finditer(text)]
    macros_file = path.endswith("_macros.svh")
    for name, index in defs:
        if name not in undefs and not macros_file:
            out.append({
                "rule_id": "macro.missing_undef",
                "severity": "warning",
                "message": f"`define {name} must be undefined at end of file",
                "location": loc(text, index),
            })
        guarded = any(pos < index for pos in guard_positions.get(name, []))
        if macros_file and not guarded:
            out.append({
                "rule_id": "macro.guard_required",
                "severity": "warning",
                "message": f"`define {name} in *_macros.svh must be wrapped with `ifndef {name}",
                "location": loc(text, index),
            })
        if not macros_file and guarded:
            out.append({
                "rule_id": "macro.no_local_guard",
                "severity": "warning",
                "message": f"local macro {name} must not use `ifndef guards",
                "location": loc(text, index),
            })
    for name in dv_macros:
        if macros_file:
            continue
        pos = text.find(name)
        out.append({
            "rule_id": "macro.dv_prefix_header_only",
            "severity": "warning",
            "message": "DV_* macros must live in dedicated *_macros.svh headers",
            "location": loc(text, pos),
        })
    return out


def _check_uvm_do(text):
    out = []
    for match in UVM_DO_RE.finditer(text):
        out.append({
            "rule_id": "seq.no_uvm_do",
            "severity": "warning",
            "message": "replace `uvm_do macros with start_item/finish_item flow",
            "location": loc(text, match.start()),
        })
    return out


def _check_scoreboard(text):
    out = []
    if not DV_EOT_RE.search(text):
        for match in SCOREBOARD_CLASS_RE.finditer(text):
            out.append({
                "rule_id": "scoreboard.dv_eot_required",
                "severity": "warning",
                "message": "scoreboard classes should call DV_EOT_PRINT_* macros in check_phase",
                "location": loc(text, match.start()),
            })
            break
    return out


def _check_program(text):
    out = []
    for match in PROGRAM_RE.finditer(text):
        out.append({
            "rule_id": "lang.no_program_construct",
            "severity": "warning",
            "message": "program blocks are disallowed; use module/interface alternatives",
            "location": loc(text, match.start()),
        })
    return out


def _check_comparison_macros(text):
    out = []
    for match in IF_COMPARE_RE.finditer(text):
        start = match.start()
        window = text[start: start + 200]
        if "DV_CHECK" in window:
            continue
        if not UVM_REPORT_CALL_RE.search(window):
            continue
        out.append({
            "rule_id": "check.dv_macro_required",
            "severity": "warning",
            "message": "use DV_CHECK_* comparison macros instead of manual if/uvm_* checks",
            "location": loc(text, start),
        })
    return out


def _check_module_macro_prefix(text):
    out = []
    for start, end, name in _module_ranges(text):
        upper = name.upper()
        prefix = f"{upper}_"
        block = text[start:end]
        offset = start
        for match in re.finditer(r"`define\s+([A-Za-z_]\w*)", block):
            macro = match.group(1)
            if macro.upper().startswith(prefix):
                continue
            location = loc(text, offset + match.start(1))
            out.append({
                "rule_id": "macro.module_prefix",
                "severity": "warning",
                "message": f"`define {macro} inside module {name} must be prefixed with {prefix}",
                "location": location,
            })
    return out


def _module_ranges(text):
    ranges = []
    for match in MODULE_RE.finditer(text):
        name = match.group(1)
        start = match.end()
        end = _find_matching_end(text, start)
        if end is not None:
            ranges.append((start, end, name))
    return ranges


def _find_matching_end(text, start):
    depth = 1
    idx = start
    while idx < len(text):
        if text.startswith("module", idx):
            depth += 1
        elif text.startswith("endmodule", idx):
            depth -= 1
            if depth == 0:
                return idx
        idx += 1
    return None
