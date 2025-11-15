import re

UNUSED_WORD = re.compile(r"\bunused\b", re.IGNORECASE)


def check(req):
    if req.get("stage") != "ast":
        return []
    payload = req.get("payload") or {}
    symbols = payload.get("symbols") or []
    line_cache = {}
    out = []
    for s in symbols:
        if s.get("class") != "net":
            continue
        reads = int(s.get("read_count", 0) or 0)
        writes = int(s.get("write_count", 0) or 0)
        if reads == 0 and writes == 0:
            if _has_unused_comment(line_cache, s.get("loc")):
                continue
            out.append({
                "rule_id": "decl.unused_net",
                "severity": "warning",
                "message": f"unused net {s.get('module','')}.{s.get('name','')}",
                "location": s.get("loc", {"line":1,"col":1,"end_line":1,"end_col":1})
            })
    return out


def _has_unused_comment(cache, loc):
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
    rest = line[start_col - 1:]
    i = 0
    length = len(rest)
    while i < length:
        if rest.startswith("//", i):
            return bool(UNUSED_WORD.search(rest[i + 2:]))
        if rest.startswith("/*", i):
            end = rest.find("*/", i + 2)
            if end == -1:
                return False
            comment = rest[i + 2:end]
            if UNUSED_WORD.search(comment):
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
