#[derive(Debug)]
#[derive(Clone, Copy)]
#[derive(PartialEq)]
pub enum TokenType {
    Null,
    Number,
    String,
    Identifier,
    Equals,
    Comma,
    Dot,
    Colon,
    SemiColon,
    Let,
    OpenParen, // (
    CloseParen, // )
    OpenBrace, // {
    CloseBrace, // }
    OpenBracket, // [
    CloseBracket, // ]
    BinOp,
    EOF
}