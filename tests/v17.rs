use vast::util::file::read_to_string;
use vast::v17::ast::*;

#[test]
fn test_decl_logic_width_32() {
    assert_eq!(
        "logic [31:0] foo".to_string(),
        Decl::Logic("foo".to_string(), Ty::Width(32)).to_string()
    );
}

#[test]
fn test_decl_logic_width_1() {
    assert_eq!(
        "logic foo".to_string(),
        Decl::Logic("foo".to_string(), Ty::Width(1)).to_string()
    );
}

#[test]
fn test_port_input_width_1() {
    assert_eq!(
        "input logic foo".to_string(),
        Port::Input(Decl::Logic("foo".to_string(), Ty::Width(1))).to_string()
    );
}

#[test]
fn test_decl_int() {
    assert_eq!(
        "int a".to_string(),
        Decl::Int("a".to_string(), Ty::Int).to_string()
    );
}

#[test]
fn test_decl_param_uint() {
    assert_eq!(
        "parameter int width = 32'd3".to_string(),
        Decl::new_param_uint("width", 3).to_string(),
    );
}

#[test]
fn test_event_ty_posedge() {
    assert_eq!("posedge".to_string(), EventTy::Posedge.to_string(),);
}

#[test]
fn test_event_ty_negedge() {
    assert_eq!("negedge".to_string(), EventTy::Negedge.to_string(),);
}

#[test]
fn test_seq_event_posedge_clock() {
    assert_eq!(
        "posedge clock".to_string(),
        Sequential::Event(EventTy::Posedge, Expr::Ref("clock".to_string())).to_string(),
    );
}

#[test]
fn test_seq_error() {
    let res = Sequential::new_error("this is an error").to_string();
    let exp = r#"$error("this is an error")"#;
    assert_eq!(res, exp);
}

#[test]
fn test_module_simple() {
    let mut module = Module::new_with_name("foo");
    module.add_input("a", 32);
    let res = module.to_string();
    let exp = r#"module foo (
    input logic [31:0] a);
endmodule
"#;
    assert_eq!(res, exp);
}

#[test]
fn test_module_empty() {
    let exp = read_to_string("regression/v17/module_empty.v");
    let res = Module::new_with_name("empty").to_string();
    assert_eq!(exp, res);
}

#[test]
fn test_module_one_input() {
    let exp = read_to_string("regression/v17/module_one_input.v");
    let mut module = Module::new_with_name("one_input");
    module.add_input("a", 5);
    let res = module.to_string();
    assert_eq!(exp, res);
}

#[test]
fn test_module_three_inputs() {
    let exp = read_to_string("regression/v17/module_four_inputs.v");
    let mut module = Module::new_with_name("four_inputs");
    module.add_input("a", 2);
    module.add_input("b", 7);
    module.add_input("c", 4);
    module.add_input("d", 1);
    let res = module.to_string();
    assert_eq!(exp, res);
}

#[test]
fn test_module_with_instances() {
    let exp = read_to_string("regression/v17/module_with_instances.v");
    let mut i0 = Instance::new("i0", "prim");
    let mut i1 = Instance::new("i1", "prim");
    let mut i2 = Instance::new("i2", "prim");
    let e0 = Expr::new_ulit_hex(4, "0");
    let e1 = Expr::new_ulit_hex(4, "8");
    let e2 = Expr::new_ulit_hex(4, "f");
    i1.add_param_str("name", "multiply");
    i2.add_param_uint("WIDTH", 3);
    i0.connect("port_a", e0);
    i1.connect("port_a", e1);
    i2.connect("port_a", e2);
    let mut module = Module::new_with_name("module_with_instances");
    module.add_instance(i0);
    module.add_instance(i1);
    module.add_instance(i2);
    let res = module.to_string();
    assert_eq!(exp, res);
}
