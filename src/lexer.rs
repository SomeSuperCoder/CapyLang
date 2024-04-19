use std::collections::{HashMap, VecDeque};

use crate::token_type::TokenType;

const NUMBERS: &str = "1234567890";

#[derive(Debug, Clone)]
pub struct Token {
    pub value: String,
    pub type_: TokenType
}

impl Token {
    pub fn new(value: String, type_: TokenType) -> Self {
        Self {
            value,
            type_
        }
    }
}

pub fn tokenize(src: String) -> Vec<Token> {
    let mut tokens = Vec::new();
    
    let mut src: VecDeque<&str> = src.split("").collect();

    // Just some Rust carazines
    src.pop_front();
    src.pop_back();

    println!("{:?}", src);

    while src.len() > 0 {
        if src[0] == "(" {
            tokens.push(
                Token::new(src.pop_front().unwrap().to_string(), TokenType::OpenParen)
            )
        } else if src[0] == ")" {
            tokens.push(
                Token::new(src.pop_front().unwrap().to_string(), TokenType::CloseParen)
            )
        } else if ["+", "-", "*", "/", "%"].contains(&src[0]) {
            tokens.push(
                Token::new(src.pop_front().unwrap().to_string(), TokenType::BinOp)
            )
        } else if src[0] == "=" {
            tokens.push(
                Token::new(src.pop_front().unwrap().to_string(), TokenType::Equals)
            )
        } else if src[0] == ";" {
            tokens.push(
                Token::new(src.pop_front().unwrap().to_string(), TokenType::SemiColon)
            )
        } else {
            // Handle multicharacter tokens
            if is_int(src[0]) {
                let mut num = String::new();
                while src.len() > 0 && is_int(src[0]) {
                    num.push_str(src.pop_front().unwrap());
                }

                tokens.push(
                    Token::new(num, TokenType::Number)
                )
            } else if is_alpha(src[0]) {
                let mut ident = String::new();
                while src.len() > 0 && is_alpha(src[0]) {
                    ident.push_str(src.pop_front().unwrap());
                }
                // chech for reserved keywords
                if let Some(token_type) = get_reserved_keywords().get(ident.as_str()) {
                    tokens.push(
                        Token::new(ident, *token_type)
                    )
                } else {
                    tokens.push(
                        Token::new(ident, TokenType::Identifier)
                    )
                }
            } else if is_skippable(src[0]) {
                src.pop_front();
            } else {
                panic!("Unrecognized character error: {}", src[0])
            }
        }
    }

    tokens.push(
        Token::new("EOF".to_string(), TokenType::EOF)
    );

    tokens
}

fn is_alpha(target: &str) -> bool {
    target.to_uppercase() != target.to_lowercase()
}

fn is_int(target: &str) -> bool {
    let c = target.chars().nth(0).unwrap();
    
    NUMBERS.chars().collect::<Vec<char>>().contains(&c)
}

fn is_skippable(target: &str) -> bool {
    [" ", "\n", "\t"].contains(&target)
}

pub fn get_reserved_keywords() -> HashMap<&'static str, TokenType> {
    let mut hm = HashMap::new();

    hm.insert("let", TokenType::Let);
    hm.insert("null", TokenType::Null);

    hm
}
