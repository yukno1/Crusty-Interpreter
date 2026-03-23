// evaluate.rs
//
// Run a Lox program

use crate::ast::{AST, Expr, Operator, Stmt};

// when evaluating, there must be some machine-representation in lox
// this enum provides this mapping
// goal of evaluator is to translate the AST into a Loxvalue
#[derive(Debug, PartialEq)]
pub enum LoxValue {
    LNil,
    LBoolean(bool),
    LNumber(f64),
    LString(String),
}

impl LoxValue {
    pub fn is_truthy(&self) -> bool {
        match self {
            LoxValue::LNil | LoxValue::LBoolean(false) => false,
            _ => true,
        }
    }
}

type Output = LoxValue;

#[derive(Debug)]
pub enum Error {
    ZeroDivision,
    UnsupportedBinaryOperation(LoxValue, Operator, LoxValue),
    UnsupportedUnaryOperation(Operator, LoxValue),
}

pub fn interpret(statements: &Vec<Stmt>) -> Result<(), Error> {
    // 0 or more statements
    todo!()
}

pub fn execute_statement(stmt: &Stmt) -> Result<(), Error> {
    // execute a single stmt
    match stmt {
        Stmt::SPrint { expr } => {
            let value = evaluate_expression(expr)?;
            // TODO: make display better
            println!("{value:?}");
        }
        Stmt::SExpression { expr } => {
            // expression evaluate, but discard result
            evaluate_expression(expr)?;
        }
    }
    Ok(()) // statement don't produce result
}

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
            use LoxValue::*;
            use Operator::*;
            let lv = evaluate_expression(left)?;
            let rv = evaluate_expression(right)?;
            match (lv, operator, rv) {
                // number ops
                (LNumber(x), OAdd, LNumber(y)) => LNumber(x + y),
                (LNumber(x), OSub, LNumber(y)) => LNumber(x - y),
                (LNumber(x), OMul, LNumber(y)) => LNumber(x * y),
                (LNumber(x), ODiv, LNumber(y)) => {
                    if y == 0.0 {
                        return Err(Error::ZeroDivision);
                    }
                    LNumber(x / y)
                }
                (LNumber(x), OLt, LNumber(y)) => LBoolean(x < y),
                (LNumber(x), OLe, LNumber(y)) => LBoolean(x <= y),
                (LNumber(x), OGt, LNumber(y)) => LBoolean(x > y),
                (LNumber(x), OGe, LNumber(y)) => LBoolean(x >= y),

                // string ops
                (LString(x), OAdd, LString(y)) => LString(format!("{x}{y}")),

                // eq works with any type
                (x, OEq, y) => LBoolean(x == y),
                (x, ONe, y) => LBoolean(x != y),
                (lv, operator, rv) => {
                    return Err(Error::UnsupportedBinaryOperation(lv, *operator, rv));
                }
            }
        }
        Expr::EUnary { operator, right } => {
            use LoxValue::*;
            use Operator::*;
            let rv = evaluate_expression(right)?;
            match (operator, rv) {
                (OSub, LNumber(x)) => LNumber(-x),
                (ONot, x) => LBoolean(!x.is_truthy()),
                (operator, rv) => {
                    return Err(Error::UnsupportedUnaryOperation(*operator, rv));
                }
            }
        }
        Expr::EGrouping { expression } => evaluate_expression(expression)?,
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
