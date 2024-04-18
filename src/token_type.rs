#[derive(Debug)]
#[derive(Clone, Copy)]
#[derive(PartialEq)]
pub enum TokenType {
    Number,
    Identifier,
    Equals,
    Let,
    OpenParen,
    CloseParen,
    BinOp,
    EOF
}