import sys,json

def respond(stage,violations):
    sys.stdout.write(json.dumps({"type":"ViolationsStage","stage":stage,"violations":violations}));sys.stdout.flush()

def index_decl_positions(ast):
    r={}
    for d in ast.get("declarations",[]):
        n=d.get("name")
        if n and n not in r:
            r[n]=(int(d.get("line",1)),int(d.get("col",1)))
    return r

def handle_ast(req):
    ast=req.get("payload",{}).get("ast",{})
    decls={d.get("name") for d in ast.get("declarations",[]) if d.get("name")}
    used={r.get("name") for r in ast.get("references",[]) if r.get("name") and r.get("kind") in ("Rhs","Lhs")}
    unused=sorted(n for n in decls-used if n)
    pos=index_decl_positions(ast)
    out=[]
    for n in unused:
        ln,col=pos.get(n,(1,1))
        out.append({"rule_id":"decl.unused","severity":"warning","message":f"'{n}' declared but never used","location":{"line":ln,"col":col,"end_line":ln,"end_col":col}})
    return out

def main():
    req=json.load(sys.stdin)
    if req.get("type")!="CheckFileStage":
        respond(req.get("stage") or "raw_text",[]);return
    s=req.get("stage")
    v=handle_ast(req) if s=="ast" else []
    respond(s,v)

if __name__=="__main__":
    main()