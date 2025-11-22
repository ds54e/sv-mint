import bisect

def byte_span_to_loc(start, end, line_starts):
    i = bisect.bisect_right(line_starts, start) - 1
    j = bisect.bisect_right(line_starts, end) - 1
    return {"line": i + 1, "col": start - line_starts[i] + 1, "end_line": j + 1, "end_col": end - line_starts[j] + 1}

class Cst:
    def __init__(self, ir):
        self.ir = ir
        self.kinds = ir.get("kind_table") or []
        self.kind_map = {name: i for i, name in enumerate(self.kinds)}
        self.toks = ir.get("tok_kind_table") or []
        self.tok_kind_map = ir.get("tok_kind_map") or {}
        self.nodes = ir.get("nodes") or []
        self.tokens = ir.get("tokens") or []
        self.text = ir.get("source_text") or ir.get("pp_text") or ""
        self.nodes_by_id = {n["id"]: n for n in self.nodes}
        self.children = {n["id"]: list(n.get("children") or []) for n in self.nodes}
        if not self.children:
            for n in self.nodes:
                p = n.get("parent")
                if p is not None:
                    self.children.setdefault(p, []).append(n["id"])

    def kind_id(self, name_or_id):
        if isinstance(name_or_id, int):
            return name_or_id
        return self.kind_map.get(name_or_id, -1)

    def tok_id(self, name_or_id):
        if isinstance(name_or_id, int):
            return name_or_id
        if name_or_id in self.tok_kind_map:
            return self.tok_kind_map[name_or_id]
        try:
            return self.toks.index(name_or_id)
        except ValueError:
            return -1

    def of_kind(self, name_or_id):
        k = self.kind_id(name_or_id)
        if k < 0:
            return []
        return [n for n in self.nodes if n.get("kind") == k]

    def tokens_in(self, node):
        ft, lt = node["first_token"], node["last_token"]
        return self.tokens[ft:lt+1]

    def loc(self, start, end):
        return byte_span_to_loc(start, end, self.ir["line_starts"])
