use vast::util::file::read_to_string;
use vast::v05::ast::{Decl, EventTy, Expr, Module, Sequential};

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
fn test_decl_param_int() {
    assert_eq!(
        "parameter width = 3".to_string(),
        Decl::new_param_int("width", 3).to_string(),
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
    module.add_param_int("width", 32);
    module.add_input("data", 4);
    let res = module.to_string();
    assert_eq!(exp, res);
}
