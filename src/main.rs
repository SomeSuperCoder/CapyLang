pub mod lexer;
pub mod token_type;
pub mod ast;
pub mod parser;

use parser::Parser;

fn main() {
    let mut parser = Parser::default();
    println!("CapyLang");
    loop {
        let mut input = String::new();

        std::io::stdin().read_line(&mut input).expect("Invalid input");

        if input.as_str() == "exit" {
            std::process::exit(0);
        }

        let program = parser.procduce_ast(input);

        println!("{:?}", program)
    }
}
