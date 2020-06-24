use vast::v05::{Module, Decl, Ty};

#[test]
fn test_decl_wire_width_32() {
    assert_eq!(
        "wire [31:0] foo".to_string(),
        Decl::Wire("foo".to_string(), Ty::Width(32)).to_string()
    );
}

#[test]
fn test_decl_wire_width_1() {
    assert_eq!(
        "wire foo".to_string(),
        Decl::Wire("foo".to_string(), Ty::Width(1)).to_string()
    );
}

#[test]
fn test_decl_reg_width_32() {
    assert_eq!(
        "reg [31:0] foo".to_string(),
        Decl::Reg("foo".to_string(), Ty::Width(32)).to_string()
    );
}

#[test]
fn test_decl_reg_width_1() {
    assert_eq!(
        "reg foo".to_string(),
        Decl::Reg("foo".to_string(), Ty::Width(1)).to_string()
    );
}

#[test]
fn test_module_with_name() {
    assert_eq!(
        "module foo ();\nendmodule".to_string(),
        Module::new_with_name("foo").to_string(),
    );
}
