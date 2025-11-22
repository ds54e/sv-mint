import re

USED_WORD = re.compile(r"\bused\b", re.IGNORECASE)
RESERVED_WORD = re.compile(r"\breserved\b", re.IGNORECASE)


def check(req):
    if req.get("stage") != "ast":
        return []
    payload = req.get("payload") or {}
    symbols = payload.get("symbols") or []
    line_cache = {}
    out = []
    for s in symbols:
        if s.get("class") != "localparam":
            continue
        refs = int(s.get("ref_count", s.get("read_count", 0) or 0) or 0)
        if refs == 0:
            if _has_usage_comment(line_cache, s.get("loc")):
                continue
            out.append(
                {
                    "rule_id": "localparams_not_left_unused",
                    "severity": "warning",
                    "message": f"unused localparam {s.get('module','')}.{s.get('name','')}",
                    "location": s.get(
                        "loc", {"line": 1, "col": 1, "end_line": 1, "end_col": 1}
                    ),
                }
            )
    return out


def _has_usage_comment(cache, loc):
    if not loc:
        return False
    path = loc.get("file")
    lines = _lines_for_file(cache, path)
    if not lines:
        return False
    line_idx = int(loc.get("line", 0) or 0) - 1
    if line_idx < 0 or line_idx >= len(lines):
        return False
    start_col = int(loc.get("col", 1) or 1)
    if start_col < 1:
        start_col = 1
    line = lines[line_idx]
    rest = line[start_col - 1 :]
    i = 0
    length = len(rest)
    while i < length:
        if rest.startswith("//", i):
            comment = rest[i + 2 :]
            return bool(USED_WORD.search(comment) or RESERVED_WORD.search(comment))
        if rest.startswith("/*", i):
            end = rest.find("*/", i + 2)
            if end == -1:
                return False
            comment = rest[i + 2 : end]
            if USED_WORD.search(comment) or RESERVED_WORD.search(comment):
                return True
            i = end + 2
            continue
        i += 1
    return False


def _lines_for_file(cache, path):
    if not path:
        return []
    if path in cache:
        return cache[path]
    try:
        with open(path, "r", encoding="utf-8", errors="replace") as handle:
            lines = handle.read().splitlines()
    except OSError:
        lines = []
    cache[path] = lines
    return lines
