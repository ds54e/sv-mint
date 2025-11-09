import sys,json

def respond(stage,violations):
    sys.stdout.write(json.dumps({"type":"ViolationsStage","stage":stage,"violations":violations}))
    sys.stdout.flush()

def handle_ast(req):
    ast=req.get("payload",{}).get("ast",{})
    out=[]
    scopes=ast.get("symbol_table",{}).get("scopes",{})
    if isinstance(scopes,dict) and scopes:
        for names in scopes.values():
            if not isinstance(names,dict):
                continue
            for n,info in names.items():
                if not isinstance(info,dict):
                    continue
                if info.get("rw_class")=="unused":
                    d=info.get("decl",{})
                    ln=int(d.get("line",1))
                    col=int(d.get("col",1))
                    out.append({"rule_id":"decl.unused","severity":"warning","message":f"'{n}' declared but never used","location":{"line":ln,"col":col,"end_line":ln,"end_col":col}})
        return out
    decls={d.get("name") for d in ast.get("declarations",[]) if d.get("name")}
    used={r.get("name") for r in ast.get("references",[]) if r.get("name") and r.get("kind") in ("Rhs","Lhs")}
    pos={}
    for d in ast.get("declarations",[]):
        n=d.get("name")
        if n and n not in pos:
            pos[n]=(int(d.get("line",1)),int(d.get("col",1)))
    for n in sorted(n for n in decls-used if n):
        ln,col=pos.get(n,(1,1))
        out.append({"rule_id":"decl.unused","severity":"warning","message":f"'{n}' declared but never used","location":{"line":ln,"col":col,"end_line":ln,"end_col":col}})
    return out

def main():
    req=json.load(sys.stdin)
    if req.get("type")!="CheckFileStage":
        respond(req.get("stage") or "raw_text",[])
        return
    s=req.get("stage")
    if s!="ast":
        respond(s,[])
        return
    v=handle_ast(req)
    respond(s,v)

if __name__=="__main__":
    main()
