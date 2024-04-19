use std::collections::HashMap;

#[derive(Debug)]
pub enum Stmt {
    Program {
        body: Vec<Stmt>
    },
    Expr {
        expr: Expr
    },
    VarDecl {
        name: String,
        value: Option<Expr>
    }
}

#[derive(Debug)]
pub enum Expr {
    BinaryExpr {
        left: Box<Expr>,
        right: Box<Expr>,
        operator: String
    },
    Identifier {
        symbol: String
    },
    NumericLiteral {
        value: f64
    },
    NullLiteral {
        value: &'static str
    },
    PropertyLiteral {
        key: String,
        value: Option<Box<Expr>>
    },
    ObjectLiteral {
        props: Vec<Box<Expr>>
    },
    AssignExpr {
        assigne: Box<Expr>,
        value: Box<Expr>
    }
}
