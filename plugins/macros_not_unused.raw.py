import re

from lib.utf8 import line_starts, point_to_loc

DEFINE_PATTERN = re.compile(r"(?m)^\s*`define\s+([A-Za-z_]\w*)")
USE_PATTERN = re.compile(r"`([A-Za-z_]\w*)")
USED_WORD = re.compile(r"\bused\b", re.IGNORECASE)
RESERVED_WORD = re.compile(r"\breserved\b", re.IGNORECASE)

def check(req):
    if req.get("stage") != "raw_text":
        return []
    payload = req.get("payload") or {}
    text = payload.get("text") or ""
    starts = line_starts(text)
    defines = [(m.group(1), m.start()) for m in DEFINE_PATTERN.finditer(text)]
    uses = {m.group(1) for m in USE_PATTERN.finditer(text)}
    out = []
    for name, start in defines:
        if name in uses:
            continue
        if _has_unused_comment(text, start):
            continue
        out.append(
            {
                "rule_id": "macros_not_unused",
                "severity": "warning",
                "message": f"macro `{name}` is defined but never used",
                "location": point_to_loc(text, start, len(name), starts),
            }
        )
    return out

def _has_unused_comment(text, offset):
    line_start = text.rfind("\n", 0, offset)
    if line_start == -1:
        line_start = 0
    else:
        line_start += 1
    end = len(text)
    cursor = line_start
    while cursor < len(text):
        nl = text.find("\n", cursor)
        if nl == -1:
            end = len(text)
            break
        line = text[cursor:nl]
        end = nl
        if not line.rstrip().endswith("\\"):
            break
        cursor = nl + 1
    segment = text[line_start:end]
    pos = segment.find("//")
    if pos != -1:
        comment = segment[pos + 2 :]
        if USED_WORD.search(comment) or RESERVED_WORD.search(comment):
            return True
    pos_block = segment.find("/*")
    if pos_block != -1:
        end = segment.find("*/", pos_block + 2)
        if end != -1:
            comment = segment[pos_block + 2 : end]
            if USED_WORD.search(comment) or RESERVED_WORD.search(comment):
                return True
    return False
