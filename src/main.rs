use vast::{v05, v17};

fn main() {
    println!("this is wire in Verilog 2005 => {}", v05::Decl::Wire("a".to_string(), 34));
    println!("Verilog 2017{}", v17::Unop::LogicalNegation);
}
