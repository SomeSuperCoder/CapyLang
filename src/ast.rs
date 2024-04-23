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

#[derive(Debug, Clone)]
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
    },
    MemberExpr {
        object: Box<Expr>,
        prop: Box<Expr>,
        computed: bool
    },
    CallExpr {
        args: Vec<Expr>,
        calle: Box<Expr>
    }
}

impl Expr {
    pub fn as_stmt(&self) -> Stmt {
        Stmt::Expr { expr: self.clone() }
    }
}
