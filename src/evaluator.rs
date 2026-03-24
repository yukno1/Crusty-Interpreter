// evaluate.rs
//
// Run a Lox program

use crate::ast::{AST, Expr, Operator, Stmt};

// when evaluating, there must be some machine-representation in lox
// this enum provides this mapping
// goal of evaluator is to translate the AST into a Loxvalue
#[derive(Debug, PartialEq, Clone)]
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

impl std::fmt::Display for LoxValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LoxValue::LNil => f.write_str("nil"),
            LoxValue::LBoolean(v) => f.write_str(&format!("{v}")),
            LoxValue::LNumber(v) => f.write_str(&format!("{v}")),
            LoxValue::LString(v) => f.write_str(&format!("{v}")),
        }
    }
}

type Output = ();

type Environment = crate::environ::Environment<LoxValue>;

#[derive(Debug)]
pub enum Error {
    ZeroDivision,
    UnsupportedBinaryOperation(LoxValue, Operator, LoxValue),
    UnsupportedUnaryOperation(Operator, LoxValue),
}

// for interactive REPL, need some way to kkeeo environment around
pub struct Interpreter {
    top_level: Environment,
}

impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter {
            top_level: Environment::new(),
        }
    }

    pub fn evaluate(&mut self, ast: AST) -> Result<Output, Error> {
        execute_statements(&ast.top, &mut self.top_level)?;
        Ok(())
    }
}

pub fn evaluate(ast: AST) -> Result<Output, Error> {
    // println!("Evaluating");
    let mut environ = Environment::new();
    execute_statements(&ast.top, &mut environ)?;
    Ok(())
}

pub fn execute_statements(statements: &Vec<Stmt>, environ: &mut Environment) -> Result<(), Error> {
    // 0 or more statements
    for stmt in statements.iter() {
        execute_statement(stmt, environ)?
    }
    Ok(())
}

pub fn execute_statement(stmt: &Stmt, environ: &mut Environment) -> Result<(), Error> {
    // execute a single stmt
    match stmt {
        Stmt::SPrint { expr } => {
            let value = evaluate_expression(expr, environ)?;
            println!("{value}");
        }
        Stmt::SExpression { expr } => {
            // expression evaluate, but discard result
            evaluate_expression(expr, environ)?;
        }
        Stmt::SVarDecl { name, initializer } => {
            let iv = match initializer {
                Some(v) => evaluate_expression(v, environ)?,
                None => LoxValue::LNil,
            };
            environ.declare(name, iv);
        }
    }
    Ok(()) // statement don't produce result
}

pub fn evaluate_expression(expr: &Expr, environ: &Environment) -> Result<LoxValue, Error> {
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
            let lv = evaluate_expression(left, environ)?;
            let rv = evaluate_expression(right, environ)?;
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
            let rv = evaluate_expression(right, environ)?;
            match (operator, rv) {
                (OSub, LNumber(x)) => LNumber(-x),
                (ONot, x) => LBoolean(!x.is_truthy()),
                (operator, rv) => {
                    return Err(Error::UnsupportedUnaryOperation(*operator, rv));
                }
            }
        }
        Expr::EGrouping { expression } => evaluate_expression(expression, environ)?,
        Expr::EVariable { name } => environ.lookup(name).unwrap().clone(),
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
