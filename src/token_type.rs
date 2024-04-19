#[derive(Debug)]
#[derive(Clone, Copy)]
#[derive(PartialEq)]
pub enum TokenType {
    Null,
    Number,
    String,
    Identifier,
    Equals,
    SemiColon,
    Let,
    OpenParen,
    CloseParen,
    BinOp,
    EOF
}