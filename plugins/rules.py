import sys,json

def emit(stage,violations):
    sys.stdout.write(json.dumps({"type":"ViolationsStage","stage":stage,"violations":violations}))
    sys.stdout.flush()

def mk(rule_id,name,line,col):
    return {"rule_id":rule_id,"severity":"warning","message":"'%s' declared but never used"%name,"location":{"line":int(line),"col":int(col),"end_line":int(line),"end_col":int(col)}}

def norm_decl_type(s):
    s=(s or "").strip().lower()
    if s in ("param","parameter","localparam"):
        return "param"
    if s in ("typedef",):
        return "typedef"
    if s in ("var","variable","net"):
        return "var"
    return s

def scope_key(sc):
    if isinstance(sc,list):
        return "::".join(sc) if sc else "::"
    if isinstance(sc,str) and sc:
        return sc
    return "::"

def collect_ref_kinds_by_binding(refs):
    m={}
    for r in refs or []:
        n=r.get("name"); k=r.get("kind"); sc=r.get("scope")
        if not n or not k: continue
        m.setdefault((scope_key(sc),n),set()).add(k)
    return m

def used_judgement(dt,kinds):
    if dt=="param":
        return ("Rhs" in kinds) or ("TypeRef" in kinds)
    if dt=="typedef":
        return ("TypeRef" in kinds) or ("Rhs" in kinds)
    return ("Rhs" in kinds) or ("Lhs" in kinds)

def main():
    req=json.load(sys.stdin)
    if req.get("type")!="CheckFileStage":
        emit(req.get("stage") or "raw_text",[]); return
    if req.get("stage")!="ast":
        emit(req.get("stage"),[]); return
    ast=req.get("payload",{}).get("ast",{}) or {}
    refs=ast.get("references",[]) or []
    refmap=collect_ref_kinds_by_binding(refs)
    out=[]
    scopes=(ast.get("symbol_table",{}) or {}).get("scopes",{}) or {}
    if isinstance(scopes,dict) and scopes:
        for scope_name,names in scopes.items():
            for n,info in (names or {}).items():
                d=(info or {}).get("decl",{}) or {}
                dt=norm_decl_type(d.get("decl_type") or info.get("decl_type"))
                kinds=set()
                if "refs" in info and isinstance(info["refs"],list):
                    for idx in info["refs"]:
                        if isinstance(idx,int) and 0<=idx<len(refs):
                            k=refs[idx].get("kind")
                            if k: kinds.add(k)
                kinds.update(refmap.get((scope_key(d.get("scope")) or scope_name or "::",n),set()))
                if not used_judgement(dt,kinds):
                    ln=int(d.get("line",1)); col=int(d.get("col",1))
                    if dt=="param": out.append(mk("decl.unused.param",n,ln,col))
                    elif dt=="typedef": out.append(mk("decl.unused.typedef",n,ln,col))
                    else: out.append(mk("decl.unused",n,ln,col))
        emit("ast",out); return
    decls=ast.get("declarations",[]) or []
    for d in decls:
        n=d.get("name"); dt=norm_decl_type(d.get("decl_type"))
        if not n: continue
        kinds=refmap.get((scope_key(d.get("scope")),n),set())
        if not used_judgement(dt,kinds):
            ln=int(d.get("line",1)); col=int(d.get("col",1))
            if dt=="param": out.append(mk("decl.unused.param",n,ln,col))
            elif dt=="typedef": out.append(mk("decl.unused.typedef",n,ln,col))
            else: out.append(mk("decl.unused",n,ln,col))
    emit("ast",out)

if __name__=="__main__": main()
