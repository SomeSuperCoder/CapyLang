use std::{cell::RefCell, rc::Rc};

use crate::{ast::{Expr, Stmt}, enviorment::Enviorment, values::Value};

pub fn eval_program(body: Vec<Stmt>, env: Rc<RefCell<Enviorment>>) -> Value {
    let mut last = Value::Null { value: "null" };

    for stmt in body {
        last = eval(stmt, env.clone());
    }

    last
}

pub fn eval(ast_node: Stmt, env: Rc<RefCell<Enviorment>>) -> Value {
    match ast_node {
        Stmt::Program { body } => {
            eval_program(body, env.clone())
        },
        Stmt::Expr { expr } => {
            match expr {
                Expr::NumericLiteral { value } => {
                    Value::Number { value: value }
                },
                Expr::NullLiteral { value } => {
                    Value::Null { value }
                },
                Expr::Identifier { symbol } => {
                    eval_identifier(symbol, env.clone())
                },
                Expr::BinaryExpr { left, right, operator } => {
                    eval_bin_expr(*left, *right, operator.as_str(), env.clone())
                },
                Expr::AssignExpr { assigne, value } => {
                    eval_assign_expr(*assigne, *value, env.clone())
                }
                _ => panic!()
            }
        },
        Stmt::VarDecl { name, value } => {
            eval_var_decl(name, value, env.clone())
        },
        _ => panic!()
    }
}

pub fn eval_bin_expr(left: Expr, right: Expr, operator: &str, env: Rc<RefCell<Enviorment>>) -> Value {
    use Value::*;
    let left = eval(Stmt::Expr { expr: left }, env.clone());
    let right = eval(Stmt::Expr { expr: right }, env.clone());

    match left {
        Number { value } => {
            let left = value;
            match right {
                Number { value } => {
                    let right = value;
                    
                    return eval_num_bin_expr(left, right, operator, env.clone())
                },
                _ => {}
            }
        },
        _ => {}
    }

    panic!("TypeError: Given values are not numbers")
}

pub fn eval_num_bin_expr(l: f64, r: f64, operator: &str, _env: Rc<RefCell<Enviorment>>) -> Value {
    let result;

    match operator {
        "+" => {
            result = l + r
        },
        "-" => {
            result = l - r
        },
        "*" => {
            result = l * r
        },
        "/" => {
            result = l / r
        },
        "%" => {
            result = l % r
        },
        _ => {
            panic!("UnexpctedError: invalid bin op operator")
        }
    }

    Value::Number { value: result }
}

pub fn eval_identifier(name: String, env: Rc<RefCell<Enviorment>>) -> Value {
    Enviorment::lookup(env.clone(), name)
}

pub fn eval_var_decl(name: String, value: Option<Expr>, env: Rc<RefCell<Enviorment>>) -> Value {
    let val;

    if let Some(value) = value {
        val = eval(Stmt::Expr { expr: value }, env.clone());
    } else {
        val = Value::Null { value: "null" }
    }

    Enviorment::declare(env.clone(), name, val)
}

pub fn eval_assign_expr(assigne: Expr, value: Expr, env: Rc<RefCell<Enviorment>>) -> Value {
    match assigne {
        Expr::Identifier { symbol } => {
            let varname = symbol;
            let value = eval(Stmt::Expr { expr: value }, env.clone());

            Enviorment::assign(env.clone(), varname, value)
        },
        _ => {
            panic!("SyntaxError: Cannot assign to {:?}", assigne);

        }
    }
}
