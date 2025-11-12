def locate(text, index, length=1):
    line = text.count("\n", 0, index) + 1
    prev = text.rfind("\n", 0, index)
    col = index + 1 if prev < 0 else index - prev
    return {
        "line": line,
        "col": col,
        "end_line": line,
        "end_col": col + length - 1,
    }


def check(req):
    if req.get("stage") != "raw_text":
        return []
    payload = req.get("payload") or {}
    text = payload.get("text") or ""
    out = []
    out.extend(find_delays(text))
    out.extend(find_always_star(text))
    out.extend(find_always_latch(text))
    out.extend(check_always_ff(text))
    out.extend(check_always_comb(text))
    return out


def find_delays(text):
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
            "rule_id": "lang.no_delays",
            "severity": "warning",
            "message": "delay (#) constructs are not permitted",
            "location": locate(text, pos),
        })
        idx = pos + 1
    return out


def find_always_star(text):
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
                out.append({
                    "rule_id": "lang.prefer_always_comb",
                    "severity": "warning",
                    "message": "use always_comb instead of always @*",
                    "location": locate(text, pos, length=len("always @*")),
                })
        idx = pos + 1
    return out


def find_always_latch(text):
    out = []
    idx = 0
    needle = "always_latch"
    while True:
        pos = text.find(needle, idx)
        if pos < 0:
            break
        out.append({
            "rule_id": "lang.no_always_latch",
            "severity": "warning",
            "message": "always_latch is discouraged; prefer flip-flops",
            "location": locate(text, pos, length=len(needle)),
        })
        idx = pos + 1
    return out


def check_always_ff(text):
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
                        "rule_id": "lang.always_ff_reset",
                        "severity": "warning",
                        "message": "always_ff should include asynchronous reset (negedge rst_n)",
                        "location": locate(text, pos, length=len(needle)),
                    })
        idx = pos + 1
    return out


def check_always_comb(text):
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
                "rule_id": "lang.always_comb_at",
                "severity": "warning",
                "message": "always_comb must not have sensitivity list",
                "location": locate(text, pos, length=len(needle)),
            })
        idx = pos + 1
    return out
