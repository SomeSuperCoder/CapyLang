pub mod lexer;
pub mod token_type;
pub mod ast;
pub mod parser;
pub mod interpreter;
pub mod values;
pub mod enviorment;

use parser::Parser;

use crate::{enviorment::Enviorment, interpreter::eval};

fn main() {
    let env = Enviorment::new(None);

    let mut parser = Parser::default();
    println!("CapyLang Repl v1.0");

    loop {
        let mut input = String::new();

        std::io::stdin().read_line(&mut input).expect("Invalid input");

        let program = parser.procduce_ast(input);
        let result = eval(program, env.clone());

        println!("{:?}", result);
    }
}
