// evaluate.rs
//
// Run a Lox program

use crate::ast::{AST, Expr};

// when evaluating, there must be some machine-representation in lox
// this enum provides this mapping
// goal of evaluator is to translate the AST into a Loxvalue
#[derive(Debug)]
pub enum LoxValue {
    LNil,
    LBoolean(bool),
    LNumber(f64),
    LString(String),
}

type Output = LoxValue;

#[derive(Debug)]
pub struct Error {}

pub fn evaluate(ast: AST) -> Result<Output, Error> {
    println!("Evaluating");
    evaluate_expression(&ast.top)
}

pub fn evaluate_expression(expr: &Expr) -> Result<LoxValue, Error> {
    Ok(match expr {
        Expr::ENum { value } => LoxValue::LNumber(value.parse().unwrap()),
        Expr::EStr { value } => LoxValue::LString(value.clone()),
        Expr::EBool { value } => LoxValue::LBoolean(*value),
        Expr::ENil => LoxValue::LNil,
        Expr::EBinary {
            left,
            operator,
            right,
        } => {
            todo!()
        }
        Expr::EUnary { operator, right } => {
            todo!()
        }
        Expr::EGrouping { expression } => {
            todo!()
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn its_alive() {
        assert_eq!(true, true);
    }
}
