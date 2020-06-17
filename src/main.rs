use vast::v05;

fn main() {
    println!("this is wire in Verilog 2005 => {}", v05::Decl::Wire("a".to_string(), 34));
}
