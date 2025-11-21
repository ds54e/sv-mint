from lib.dv_helpers import loc, raw_text_inputs

CACHE_KEY = "__lang_construct_ruleset"


def violations_for(req, rule_id):
    table = evaluate(req)
    return list(table.get(rule_id) or [])


def evaluate(req):
    cached = req.get(CACHE_KEY)
    if cached is not None:
        return cached
    inputs = raw_text_inputs(req)
    if not inputs:
        req[CACHE_KEY] = {}
        return req[CACHE_KEY]
    text, _ = inputs
    out = []
    out.extend(_find_delays(text))
    out.extend(_find_always_star(text))
    out.extend(_find_always_latch(text))
    out.extend(_check_always_ff(text))
    out.extend(_check_always_comb(text))
    table = {}
    for item in out:
        table.setdefault(item["rule_id"], []).append(item)
    req[CACHE_KEY] = table
    return table


def _find_delays(text):
    out = []
    idx = 0
    while True:
        pos = text.find("#", idx)
        if pos < 0:
            break
        j = pos + 1
        while j < len(text) and text[j].isspace():
            j += 1
        if j < len(text) and text[j] == "(":
            idx = j
            continue
        out.append({
            "rule_id": "lang_no_delays",
            "severity": "warning",
            "message": "delay (#) constructs are not permitted",
            "location": loc(text, pos),
        })
        idx = pos + 1
    return out


def _find_always_star(text):
    out = []
    idx = 0
    needle = "always"
    while True:
        pos = text.find(needle, idx)
        if pos < 0:
            break
        after = pos + len(needle)
        while after < len(text) and text[after].isspace():
            after += 1
        if after < len(text) and text[after] == "@":
            after += 1
            while after < len(text) and text[after].isspace():
                after += 1
            if after < len(text) and text[after] == "*":
                pass
        idx = pos + 1
    return out


def _find_always_latch(text):
    out = []
    needle = "always_latch"
    idx = 0
    while True:
        pos = text.find(needle, idx)
        if pos < 0:
            break
        out.append({
            "rule_id": "lang_no_always_latch",
            "severity": "warning",
            "message": "always_latch is discouraged; prefer flip-flops",
            "location": loc(text, pos),
        })
        idx = pos + 1
    return out


def _check_always_ff(text):
    out = []
    needle = "always_ff"
    idx = 0
    while True:
        pos = text.find(needle, idx)
        if pos < 0:
            break
        start = text.find("@", pos)
        if start >= 0:
            end = text.find(")", start)
            if end > start:
                window = text[start:end]
                if "negedge" not in window:
                    out.append({
                        "rule_id": "lang_always_ff_require_async_reset",
                        "severity": "warning",
                        "message": "always_ff should include asynchronous reset (negedge rst_n)",
                        "location": loc(text, pos),
                    })
        idx = pos + 1
    return out


def _check_always_comb(text):
    out = []
    needle = "always_comb"
    idx = 0
    while True:
        pos = text.find(needle, idx)
        if pos < 0:
            break
        after = pos + len(needle)
        while after < len(text) and text[after].isspace():
            after += 1
        if after < len(text) and text[after] == "@":
            out.append({
                "rule_id": "lang_always_comb_no_sensitivity",
                "severity": "warning",
                "message": "always_comb must not have sensitivity list",
                "location": loc(text, pos),
            })
        idx = pos + 1
    return out
