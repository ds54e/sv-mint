import sys,json

def emit(stage,violations):
    sys.stdout.write(json.dumps({"type":"ViolationsStage","stage":stage,"violations":violations}))
    sys.stdout.flush()

def mk_violation(rule_id,name,line,col):
    return {"rule_id":rule_id,"severity":"warning","message":"'%s' declared but never used"%name,"location":{"line":int(line),"col":int(col),"end_line":int(line),"end_col":int(col)}}

def from_symbol_table(ast):
    out=[]
    scopes=ast.get("symbol_table",{}).get("scopes",{})
    if not isinstance(scopes,dict) or not scopes:
        return out,False
    for names in scopes.values():
        if not isinstance(names,dict):
            continue
        for n,info in names.items():
            if not isinstance(info,dict):
                continue
            rw=info.get("rw_class")
            d=info.get("decl",{})
            dt=(d.get("decl_type") or info.get("decl_type") or "").lower()
            if rw=="unused":
                if dt=="param":
                    out.append(mk_violation("decl.unused.param",n,d.get("line",1),d.get("col",1)))
                elif dt=="typedef":
                    out.append(mk_violation("decl.unused.typedef",n,d.get("line",1),d.get("col",1)))
                else:
                    out.append(mk_violation("decl.unused",n,d.get("line",1),d.get("col",1)))
    return out,True

def from_lists(ast):
    out=[]
    decls=ast.get("declarations",[])
    refs=ast.get("references",[])
    used_rhs={r.get("name") for r in refs if r.get("name") and r.get("kind") in ("Rhs","Lhs")}
    used_type={r.get("name") for r in refs if r.get("name") and r.get("kind")=="TypeRef"}
    pos={}
    dtype={}
    for d in decls:
        n=d.get("name")
        if not n:
            continue
        if n not in pos:
            pos[n]=(d.get("line",1),d.get("col",1))
            dtype[n]=(d.get("decl_type") or "").lower()
    for d in decls:
        n=d.get("name")
        if not n:
            continue
        dt=(d.get("decl_type") or "").lower()
        if dt=="param":
            if n not in used_rhs and n not in used_type:
                ln,col=pos.get(n,(1,1))
                out.append(mk_violation("decl.unused.param",n,ln,col))
        elif dt=="typedef":
            if n not in used_type:
                ln,col=pos.get(n,(1,1))
                out.append(mk_violation("decl.unused.typedef",n,ln,col))
        else:
            if n not in used_rhs:
                ln,col=pos.get(n,(1,1))
                out.append(mk_violation("decl.unused",n,ln,col))
    return out

def main():
    req=json.load(sys.stdin)
    if req.get("type")!="CheckFileStage":
        emit(req.get("stage") or "raw_text",[])
        return
    stage=req.get("stage")
    if stage!="ast":
        emit(stage,[])
        return
    ast=req.get("payload",{}).get("ast",{})
    out,ok=from_symbol_table(ast)
    if not ok:
        out=from_lists(ast)
    emit(stage,out)

if __name__=="__main__":
    main()
