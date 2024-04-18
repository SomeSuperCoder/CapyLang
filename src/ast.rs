#[derive(Debug)]
pub enum Stmt {
    Program { body: Vec<Stmt> },
    Expr { expr: Expr }
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
    }
}
