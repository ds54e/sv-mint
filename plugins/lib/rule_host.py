import importlib.util
import json
import sys
from pathlib import Path


def prepend_paths():
    base = Path(__file__).resolve().parent.parent
    lib = base / "lib"
    for entry in (str(base), str(lib)):
        if entry not in sys.path:
            sys.path.insert(0, entry)


def load_module(path, index):
    spec = importlib.util.spec_from_file_location(f"sv_mint_rule_{index}", path)
    if spec is None or spec.loader is None:
        raise RuntimeError(f"cannot load {path}")
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


def main():
    prepend_paths()
    first = sys.stdin.readline()
    if not first:
        return
    init = json.loads(first)
    scripts = init.get("scripts") or []
    modules = []
    for idx, script in enumerate(scripts):
        mod = load_module(Path(script), idx)
        modules.append(mod)
    print(json.dumps({"type": "ready"}))
    sys.stdout.flush()
    for line in sys.stdin:
        if not line:
            break
        req = json.loads(line)
        kind = req.get("kind")
        if kind == "shutdown":
            break
        results = []
        error = None
        for module in modules:
            handler = getattr(module, "check", None)
            if handler is None:
                continue
            try:
                out = handler(req)
            except Exception as exc:
                error = {"type": "error", "detail": str(exc)}
                break
            if out:
                results.extend(out)
        if error:
            print(json.dumps(error))
            sys.stdout.flush()
            break
        print(json.dumps({"type": "violations", "violations": results}))
        sys.stdout.flush()


if __name__ == "__main__":
    main()
