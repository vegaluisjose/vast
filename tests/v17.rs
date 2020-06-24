use vast::v17::{Module, Decl, Ty, Port, EventTy, Sequential, Expr};

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
    assert_eq!("int a".to_string(), Decl::Int("a".to_string(), Ty::Int).to_string());
}

#[test]
fn test_event_ty_posedge() {
    assert_eq!(
        "posedge".to_string(),
        EventTy::Posedge.to_string(),
    );
}

#[test]
fn test_event_ty_negedge() {
    assert_eq!(
        "negedge".to_string(),
        EventTy::Negedge.to_string(),
    );
}

#[test]
fn test_sequential_event_posedge_clock() {
    assert_eq!(
        "posedge clock".to_string(),
        Sequential::Event(EventTy::Posedge, Expr::Ref("clock".to_string())).to_string(),
    );
}

#[test]
fn test_module_with_name() {
    assert_eq!(
        "module foo ();\nendmodule".to_string(),
        Module::new_with_name("foo").to_string(),
    );
}
