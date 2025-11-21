import re

from lib.raw_text_helpers import byte_loc, raw_inputs


def check(req):
    inputs = raw_inputs(req)
    if not inputs:
        return []
    text, _ = inputs
    out = []
    for start, end, name in _module_ranges(text):
        prefix = f"{name.upper()}_"
        block = text[start:end]
        offset = start
        for match in re.finditer(r"`define\s+([A-Za-z_]\w*)", block):
            macro = match.group(1)
            if macro.upper().startswith(prefix):
                continue
            out.append({
                "rule_id": "macros_use_module_prefix",
                "severity": "warning",
                "message": f"`define {macro} inside module {name} must be prefixed with {prefix}",
                "location": byte_loc(text, offset + match.start(1)),
            })
    return out


def _module_ranges(text):
    ranges = []
    for match in re.finditer(r"\bmodule\s+([A-Za-z_]\w*)", text, re.IGNORECASE):
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
