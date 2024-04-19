pub mod lexer;
pub mod token_type;
pub mod ast;
pub mod parser;
pub mod interpreter;
pub mod values;
pub mod enviorment;

use std::fs;

use parser::Parser;

use crate::{enviorment::Enviorment, interpreter::eval};
use std::rc::Rc;
use std::cell::RefCell;

fn main() {
    let env = Enviorment::new(None);

    let args: Vec<String> = std::env::args().collect();

    if let Some(filename) = args.get(1) {
        let code = fs::read_to_string(filename).unwrap();
        exec(code, env.clone());
        std::process::exit(0);
    }

    println!("CapyLang Repl v1.0");

    loop {
        let mut input = String::new();

        std::io::stdin().read_line(&mut input).expect("Invalid input");
        exec(input, env.clone());
    }
}

fn exec(code: String, env: Rc<RefCell<Enviorment>>) {
    let mut parser = Parser::default();
    let program = parser.procduce_ast(code);
    println!("{:?}", program);
    let result = eval(program, env.clone());

    println!("{:?}", result);
}
