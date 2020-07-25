use vast::v05::ast::Module;

fn main() {
    let mut m = Module::new_with_name("foo");
    m.add_param_int("width", 32);
    m.add_input("data", 4);
    println!("{}", m)
}
