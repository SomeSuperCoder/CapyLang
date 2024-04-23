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
        match self.at().type_ {
            TokenType::Let => {
                self.parse_var_decl()
            },
            _ => {
                Stmt::Expr { expr: self.parse_expr() }
            }
        }
    }

    fn parse_var_decl(&mut self) -> Stmt {
        self.eat(); // Remove the let keyword

        let name = self.expect(TokenType::Identifier).value;

        if self.at().type_ == TokenType::SemiColon {
            self.eat();
            return Stmt::VarDecl { name: name, value: None }
        }

        self.expect(TokenType::Equals);

        let value_expr = self.parse_expr();

        Stmt::VarDecl { name: name, value: Some(value_expr) }
    }

    fn parse_expr(&mut self) -> Expr {
        self.parse_assign_expr()
    }

    fn parse_assign_expr(&mut self) -> Expr {
        let left = self.parse_object_expr();

        if self.at().type_ == TokenType::Equals {
            self.eat();

            let value = self.parse_assign_expr();

            return Expr::AssignExpr { assigne: Box::new(left), value: Box::new(value) }
        }

        left
    }

    fn parse_object_expr(&mut self) -> Expr {
        if self.at().type_ == TokenType::OpenBrace {
            self.eat();

            let mut props: Vec<Box<Expr>> = Vec::new();

            while self.not_eof() && self.at().type_ != TokenType::CloseBrace {
                let key = self.expect(TokenType::Identifier).value;

                if self.at().type_ == TokenType::Comma {
                    self.eat();

                    props.push(
                        Box::new(
                            Expr::PropertyLiteral { key: key, value: None }
                        )
                    );
                    continue;
                } else if self.at().type_ == TokenType::CloseBrace {
                    props.push(
                        Box::new(
                            Expr::PropertyLiteral { key: key, value: None }
                        )
                    );
                    continue;
                }

                self.expect(TokenType::Colon);

                let value = self.parse_expr();

                props.push(
                    Box::new(
                        Expr::PropertyLiteral { key, value: Some(Box::new(value)) }
                    )
                );

                if self.at().type_ != TokenType::CloseBrace {
                    self.expect(TokenType::Comma);
                }
            }

            self.expect(TokenType::CloseBrace);

            Expr::ObjectLiteral { props }
        } else {
            self.parse_additive_expr()
        }
    }

    fn parse_additive_expr(&mut self) -> Expr {
        let mut left = self.parse_mult_expr();

        while ["+", "-"].contains(&self.at().value.as_str()) {
            let op = self.eat().value;
            let right = self.parse_mult_expr();
            left = Expr::BinaryExpr {
                left: Box::new(left),
                right: Box::new(right),
                operator: op
            };
        }

        left
    }

    fn parse_mult_expr(&mut self) -> Expr {
        let mut left = self.parse_call_member_expr();

        while ["*", "/", "%"].contains(&self.at().value.as_str()) {
            let op = self.eat().value;
            let right = self.parse_call_member_expr();
            left = Expr::BinaryExpr {
                left: Box::new(left),
                right: Box::new(right),
                operator: op
            };
        }

        left
    }
    
    // Call member zone
    fn parse_call_member_expr(&mut self) -> Expr {
        let member = self.parse_member_expr();

        if self.at().type_ == TokenType::OpenParen {
            self.prase_call_expr(member)
        } else {
            member
        }
    }

    fn prase_call_expr(&mut self, calle: Expr) -> Expr {
        let mut call_expr = Expr::CallExpr { args: self.parse_args(), calle: Box::new(calle) };

        if self.at().type_ == TokenType::OpenParen {
            call_expr = self.prase_call_expr(call_expr);
        }

        call_expr
    }

    fn parse_args(&mut self) -> Vec<Expr> {
        self.expect(TokenType::OpenParen);

        let args;
        if self.at().type_ == TokenType::CloseParen {
            args = Vec::new()
        } else {
            args = self.parse_args_list()
        }

        self.expect(TokenType::CloseParen);

        args
    }

    fn parse_args_list(&mut self) -> Vec<Expr> {
        let mut args = Vec::new();

        args.push(self.parse_assign_expr());


        while self.at().type_ != TokenType::Comma {
            self.eat(); // Is this currect?
            args.push(self.parse_assign_expr());
        }

        args
    }

    fn parse_member_expr(&mut self) -> Expr {
        let mut obj = self.parse_primary_expr();

        while self.at().type_ == TokenType::Dot || self.at().type_ == TokenType::OpenBracket {
            let op = self.eat();
            let prop;
            let computed;

            // not-computed aka obj.expr
            if op.type_ == TokenType::Dot {
                computed = false;
                // get identifyer
                prop = self.parse_primary_expr();

                if let Expr::Identifier { symbol: _ } = &prop {} else {
                    panic!("Can only access propertes of varaibles")
                }
            } else { // computed value aka obj["aaaa"]
                computed = true;
                prop = self.parse_expr();
                self.expect(TokenType::CloseBracket);
            }

            obj = Expr::MemberExpr { object: Box::new(obj), prop: Box::new(prop), computed }
        }

        obj
    }
    // ==========================

    fn parse_primary_expr(&mut self) -> Expr {
        let tk = self.at().type_;

        match tk {
            TokenType::Identifier => {
                Expr::Identifier { symbol: self.eat().value }
            },
            TokenType::Null => {
                self.eat();
                Expr::NullLiteral { value: "null" }
            },
            TokenType::Number => {
                Expr::NumericLiteral { value: self.eat().value.parse().expect("Error parsing float") }
            },
            TokenType::OpenParen => {
                self.eat(); // Eat open paren
                let value = self.parse_expr();
                self.expect(TokenType::CloseParen); // Eat close paren

                value
            }
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

    fn expect(&mut self, type_: TokenType) -> Token {
        let prev = self.tokens.pop_front();

        if let Some(prev) = prev {
            if prev.type_ != type_ {
                panic!("ParseError: expcted {:?}, found {:?}", type_, prev.type_)
            }
            prev
        } else {
            panic!("ParseError: EOF while parsing, expcted {:?}", type_)
        }
    }
}


// Order of Precedence
// AssignmentExpr
// Object
// AdditiveExpr
// MultiplicitaveExpr
// Call
// Member
// UndaryExpr
// PrimaryExpr
