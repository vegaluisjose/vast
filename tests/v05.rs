use vast::util::file::read_to_string;
use vast::v05::ast::*;

#[test]
fn test_expr_str() {
    assert_eq!(
        r#""multiply""#.to_string(),
        Expr::new_str("multiply").to_string()
    );
}

#[test]
fn test_expr_ulit_bin() {
    assert_eq!(
        "4'b1000".to_string(),
        Expr::new_ulit_bin(4, "1000").to_string()
    );
}

#[test]
fn test_expr_ulit_hex() {
    assert_eq!("8'hff".to_string(), Expr::new_ulit_hex(8, "ff").to_string());
}

#[test]
fn test_expr_ulit_dec() {
    assert_eq!(
        "16'd78".to_string(),
        Expr::new_ulit_dec(16, "78").to_string()
    );
}

#[test]
fn test_expr_ref() {
    assert_eq!("a".to_string(), Expr::new_ref("a").to_string());
}

#[test]
fn test_decl_wire_width_32() {
    assert_eq!(
        "wire [31:0] foo".to_string(),
        Decl::new_wire("foo", 32).to_string()
    );
}

#[test]
fn test_decl_wire_width_1() {
    assert_eq!("wire foo".to_string(), Decl::new_wire("foo", 1).to_string());
}

#[test]
fn test_decl_reg_width_32() {
    assert_eq!(
        "reg [31:0] foo".to_string(),
        Decl::new_reg("foo", 32).to_string()
    );
}

#[test]
fn test_decl_reg_width_1() {
    assert_eq!("reg foo".to_string(), Decl::new_reg("foo", 1).to_string());
}

#[test]
fn test_decl_param_uint() {
    assert_eq!(
        "parameter width = 32'd3".to_string(),
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
fn test_sequential_event_posedge_clock() {
    assert_eq!(
        "posedge clock".to_string(),
        Sequential::Event(EventTy::Posedge, Expr::Ref("clock".to_string())).to_string(),
    );
}

#[test]
fn test_module_empty() {
    let exp = read_to_string("regression/v05/module_empty.v");
    let res = Module::new_with_name("empty").to_string();
    assert_eq!(exp, res);
}

#[test]
fn test_module_one_input() {
    let exp = read_to_string("regression/v05/module_one_input.v");
    let mut module = Module::new_with_name("one_input");
    module.add_input("a", 5);
    let res = module.to_string();
    assert_eq!(exp, res);
}

#[test]
fn test_module_three_inputs() {
    let exp = read_to_string("regression/v05/module_three_inputs.v");
    let mut module = Module::new_with_name("three_inputs");
    module.add_input("a", 5);
    module.add_input("b", 61);
    module.add_input("c", 1);
    let res = module.to_string();
    assert_eq!(exp, res);
}

#[test]
fn test_module_one_param() {
    let exp = read_to_string("regression/v05/module_one_param.v");
    let mut module = Module::new_with_name("one_param");
    module.add_param_uint("width", 32);
    module.add_input("data", 4);
    let res = module.to_string();
    assert_eq!(exp, res);
}

#[test]
fn test_module_two_params() {
    let exp = read_to_string("regression/v05/module_two_params.v");
    let mut module = Module::new_with_name("two_params");
    module.add_param_uint("width", 4);
    module.add_param_uint("length", 8);
    module.add_input("data", 4);
    let res = module.to_string();
    assert_eq!(exp, res);
}

#[test]
fn test_module_mix_params() {
    let exp = read_to_string("regression/v05/module_mix_params.v");
    let mut module = Module::new_with_name("mix_params");
    module.add_param_uint("width", 4);
    module.add_param_uint("length", 8);
    module.add_param_str("name", "foo");
    module.add_input("data", 4);
    let res = module.to_string();
    assert_eq!(exp, res);
}

#[test]
fn test_module_with_instances() {
    let exp = read_to_string("regression/v05/module_with_instances.v");
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
