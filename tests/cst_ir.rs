use sv_mint::io::textutil::normalize_lf;
use sv_mint::sv::cst_ir::{CstIr, NodeRec};
use sv_mint::sv::driver::{SvDriver, SvParserCfg};

#[test]
fn parameter_missing_type_links_to_implicit_node() {
    let ir = load_ir("fixtures/rules/parameter_has_type/bad_missing_type.sv");
    let param = find_node(&ir, "ParameterDeclaration");
    let ty = field_id(param, "type");
    assert!(is_implicit_type(&ir, ty, None));
}

#[test]
fn parameter_with_type_has_explicit_type_field() {
    let ir = load_ir("fixtures/rules/parameter_has_type/good.sv");
    let params = find_all_nodes(&ir, "ParameterDeclaration");
    assert!(!params.is_empty());
    for param in params {
        let ty = field_id(param, "type");
        assert!(!is_implicit_type(&ir, ty, None));
    }
}

#[test]
fn function_ports_expose_explicit_types() {
    let ir = load_ir("fixtures/cst_ir/functions_explicit_arg_types_good.sv");
    let func = find_node(&ir, "FunctionDeclaration");
    let ret = field_id(func, "return_type");
    assert!(!is_implicit_type(&ir, ret, None));
    let ports = func
        .fields
        .get("ports")
        .and_then(|v| v.as_array())
        .cloned()
        .unwrap_or_default();
    assert!(!ports.is_empty());
    for port in ports {
        let ty = port.get("type").and_then(|v| v.as_u64()).unwrap() as u32;
        let name_token = port.get("name_token").and_then(|v| v.as_u64()).map(|v| v as u32);
        assert!(!is_implicit_type(&ir, ty, name_token));
        assert!(name_token.is_some());
    }
}

#[test]
fn function_missing_types_mark_ports_and_return() {
    let ir = load_ir("fixtures/cst_ir/functions_explicit_return_type_bad.sv");
    let func = find_node(&ir, "FunctionDeclaration");
    let ret = field_id(func, "return_type");
    assert!(is_implicit_type(&ir, ret, None));
    let ports = func
        .fields
        .get("ports")
        .and_then(|v| v.as_array())
        .cloned()
        .unwrap_or_default();
    assert_eq!(ports.len(), 2);
    for port in ports {
        let ty = port.get("type").and_then(|v| v.as_u64()).unwrap() as u32;
        let name_token = port.get("name_token").and_then(|v| v.as_u64()).map(|v| v as u32);
        assert!(is_implicit_type(&ir, ty, name_token));
        assert!(name_token.is_some());
    }
}

#[test]
fn directives_capture_default_nettype() {
    let ir = load_ir("fixtures/cst_ir/default_nettype_pair.sv");
    let defaults: Vec<_> = ir.directives.iter().filter(|d| d.kind == "default_nettype").collect();
    assert_eq!(defaults.len(), 2);
    assert_eq!(defaults[0].value.as_deref(), Some("none"));
    assert_eq!(defaults[1].value.as_deref(), Some("wire"));
}

#[test]
fn always_events_track_or_separators() {
    let ir = load_ir("fixtures/rules/sensitivity_list_uses_commas/bad.sv");
    let always = find_node(&ir, "AlwaysConstruct");
    let fields = &always.fields;
    assert_eq!(fields.get("always_kind").and_then(|v| v.as_str()), Some("ff"));
    let events = fields
        .get("events")
        .and_then(|v| v.as_array())
        .cloned()
        .unwrap_or_default();
    assert!(events
        .iter()
        .any(|e| e.get("separator").and_then(|s| s.as_str()) == Some("or")));
}

#[test]
fn case_flags_indicate_unique_without_default() {
    let ir = load_ir("fixtures/rules/case_has_default_branch/good.sv");
    let cases = find_all_nodes(&ir, "CaseStatement");
    let case = cases
        .into_iter()
        .find(|c| c.fields.get("is_unique").and_then(|v| v.as_bool()) == Some(true))
        .unwrap();
    assert_eq!(case.fields.get("has_default").and_then(|v| v.as_bool()), Some(false));
    assert_eq!(case.fields.get("is_unique").and_then(|v| v.as_bool()), Some(true));
}

#[test]
fn instance_connections_mark_positional_ports() {
    let ir = load_ir("fixtures/cst_ir/instances_use_named_ports_bad.sv");
    let insts = find_all_nodes(&ir, "HierarchicalInstance");
    assert_eq!(insts.len(), 3);
    let positional = insts.iter().any(|n| {
        n.fields
            .get("connections")
            .and_then(|c| c.as_array())
            .map(|a| {
                a.iter()
                    .any(|e| e.get("named").and_then(|v| v.as_bool()) == Some(false))
            })
            .unwrap_or(false)
    });
    assert!(positional);
    let conns = insts[0]
        .fields
        .get("connections")
        .and_then(|c| c.as_array())
        .cloned()
        .unwrap_or_default();
    assert!(conns.iter().any(|e| e.get("expr").is_some()));
}

fn load_ir(path: &str) -> CstIr {
    let raw = std::fs::read_to_string(path).unwrap();
    let normalized = normalize_lf(raw.clone());
    let driver = SvDriver::new(&SvParserCfg::default());
    let artifacts = driver
        .parse_text(&raw, &normalized, &std::path::PathBuf::from(path))
        .unwrap();
    artifacts.cst_ir.unwrap()
}

fn find_node<'a>(ir: &'a CstIr, name: &str) -> &'a NodeRec {
    let kind = ir.kind_table.iter().position(|k| k == name).unwrap() as u16;
    ir.nodes.iter().find(|n| n.kind == kind).unwrap()
}

fn find_all_nodes<'a>(ir: &'a CstIr, name: &str) -> Vec<&'a NodeRec> {
    let kind = ir.kind_table.iter().position(|k| k == name).unwrap() as u16;
    ir.nodes.iter().filter(|n| n.kind == kind).collect()
}

fn field_id(node: &NodeRec, key: &str) -> u32 {
    node.fields.get(key).and_then(|v| v.as_u64()).unwrap() as u32
}

fn kind_name(ir: &CstIr, id: u32) -> &str {
    let node = node_by_id(ir, id);
    ir.kind_table[node.kind as usize].as_str()
}

fn node_by_id(ir: &CstIr, id: u32) -> &NodeRec {
    ir.nodes.iter().find(|n| n.id == id).unwrap()
}

fn is_implicit_type(ir: &CstIr, type_id: u32, name_token: Option<u32>) -> bool {
    let node = node_by_id(ir, type_id);
    let kind = kind_name(ir, type_id);
    if kind == "ImplicitDataType" {
        let has_signing = node.children.iter().any(|id| kind_name(ir, *id).contains("Signing"));
        return !has_signing;
    }
    if kind == "DataType" {
        if let Some(tok) = name_token {
            return node.first_token == tok && node.last_token == tok;
        }
    }
    false
}
