mod parser;

fn main() {
    println!("{:?}", parser::json("{\"name\":\"lw\"}"));
}
