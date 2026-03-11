// evaluate.rs
//
// Run a Lox program

use crate::ast::{AST, Expr};

// when evaluating, there must be some machine-representation in lox
// this enum provides this mapping
// goal of evaluator is to translate the AST into a Loxvalue
pub enum LoxValue {
    LNil,
    LBoolean(bool),
    LNumber(f64),
}

type Output = LoxValue;

#[derive(Debug)]
pub struct Error {}

pub fn evaluate(_ast: AST) -> Result<Output, Error> {
    println!("Evaluating");
    Ok(LoxValue::LNil)
}

pub fn evaluate_expression(expr: &Expr) -> Result<LoxValue, Error> {
    Ok(match expr {
        Expr::ENum { value } => {
            todo!()
        }
        Expr::EStr { value } => {
            todo!()
        }
        Expr::EBool { value } => {
            todo!()
        }
        Expr::ENil => {
            todo!()
        }
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
