use std::collections::VecDeque;

use crate::{ast::{Expr, Stmt}, lexer::{tokenize, Token}, token_type::TokenType};

#[derive(Default)]
pub struct Parser {
    tokens: VecDeque<Token>
}

impl Parser {
    pub fn procduce_ast(&mut self, src: String) -> Stmt {
        self.tokens = tokenize(src).into();
        let mut body  = Vec::new();

        // Prase until the end of file
        while self.not_eof() {
            body.push(self.parse_stmt())
        }

        Stmt::Program { body: body }
    }

    fn not_eof(&self) -> bool {
        self.tokens[0].type_ != TokenType::EOF
    }

    fn parse_stmt(&mut self) -> Stmt {
        Stmt::Expr { expr: self.parse_expr() }
    }

    fn parse_expr(&mut self) -> Expr {
        self.parse_additive_expr()
    }

    fn parse_additive_expr(&mut self) -> Expr {
        let mut left = self.parse_primary_expr();

        while ["+", "-"].contains(&self.at().value.as_str()) {
            let op = self.eat().value;
            let right = self.parse_primary_expr();
            left = Expr::BinaryExpr {
                left: Box::new(left),
                right: Box::new(right),
                operator: op
            };
        }

        left
    }

    fn parse_primary_expr(&mut self) -> Expr {
        let tk = self.at().type_;

        match tk {
            TokenType::Identifier => {
                Expr::Identifier { symbol: self.eat().value }
            },
            TokenType::Number => {
                Expr::NumericLiteral { value: self.eat().value.parse().expect("Error parsing float") }
            },
            _ => {
                panic!("Unexpected token found while parsing: {:?}", tk)
            }
        }
    }

    fn at(&self) -> Token {
        self.tokens[0].clone()
    }

    fn eat(&mut self) -> Token {
        self.tokens.pop_front().unwrap()
    }
}


// Order of Precedence
// AssignmentExpr
// MemberExpr
// Function call
// Logical Expr
// CompirationExpr
// AdditiveExpr
// MultiplicitaveExpr
// UndaryExpr
// PrimaryExpr
