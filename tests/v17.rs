use vast::v17::{Decl, Ty, Port};

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
