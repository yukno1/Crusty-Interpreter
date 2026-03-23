// ast should be a stand-alone ds that has no dependence on ds defined in tokenizer

// top-level representation of ast for entire program
#[derive(Debug, PartialEq)]
pub struct AST {
    pub top: Vec<Stmt>,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Operator {
    OAdd,
    OSub,
    OMul,
    ODiv,
    OLt,
    OLe,
    OGt,
    OGe,
    OEq,
    ONe,
    OAnd,
    ONot,
    OOr,
}

impl std::fmt::Display for Operator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Operator::*;
        f.write_str(match self {
            OAdd => "+",
            OSub => "-",
            OMul => "*",
            ODiv => "/",
            OLt => "<",
            OLe => "<=",
            OGt => ">",
            OGe => ">=",
            OEq => "==",
            ONe => "!=",
            OAnd => "and",
            ONot => "!",
            OOr => "or",
        })
    }
}

#[derive(Debug, PartialEq)]
pub enum Expr {
    // store number as strings representing the way
    // in which tthe number is actually written in source code
    ENum {
        value: String,
    },
    EStr {
        value: String,
    },
    EBool {
        value: bool,
    },
    ENil,
    EBinary {
        left: Box<Expr>,
        operator: Operator,
        right: Box<Expr>,
    },
    EUnary {
        operator: Operator,
        right: Box<Expr>,
    },
    EGrouping {
        expression: Box<Expr>,
    },
    EVariable {
        name: String,
    },
}

impl Expr {
    pub fn num(value: impl Into<String>) -> Expr {
        Expr::ENum {
            value: value.into(),
        }
    }
    pub fn str(value: impl Into<String>) -> Expr {
        Expr::EStr {
            value: value.into(),
        }
    }
    pub fn bool(value: bool) -> Expr {
        Expr::EBool { value }
    }
    pub fn nil() -> Expr {
        Expr::ENil
    }

    pub fn binary(left: Expr, operator: Operator, right: Expr) -> Expr {
        Expr::EBinary {
            left: left.into(),
            operator,
            right: right.into(),
        }
        // into() put value into a Box
    }

    pub fn unary(operator: Operator, right: Expr) -> Expr {
        Expr::EUnary {
            operator,
            right: right.into(),
        }
        // into() put value into a Box
    }

    pub fn grouping(expr: Expr) -> Expr {
        Expr::EGrouping {
            expression: expr.into(),
        }
    }

    pub fn variable(name: impl Into<String>) -> Expr {
        Expr::EVariable { name: name.into() }
    }
}

// statement
#[derive(Debug, PartialEq)]
pub enum Stmt {
    SPrint { expr: Expr },
    SExpression { expr: Expr },
    SVarDecl { name: String, initializer: Expr },
}

impl Stmt {
    pub fn print(e: Expr) -> Stmt {
        Stmt::SPrint { expr: e }
    }

    pub fn expression(e: Expr) -> Stmt {
        Stmt::SExpression { expr: e }
    }

    pub fn var(name: impl Into<String>, initializer: Expr) -> Stmt {
        Stmt::SVarDecl {
            name: name.into(),
            initializer,
        }
    }
}

pub fn format_op(op: &Operator) -> &'static str {
    match op {
        Operator::OAdd => "+",
        Operator::OSub => "-",
        Operator::OMul => "*",
        Operator::ODiv => "/",
        Operator::OLt => "<",
        Operator::OLe => "<=",
        Operator::OGt => ">",
        Operator::OGe => ">=",
        Operator::OEq => "==",
        Operator::ONe => "!=",
        Operator::OAnd => "and",
        Operator::ONot => "!",
        Operator::OOr => "or",
    }
}

pub fn format_expr(expr: &Expr) -> String {
    match expr {
        Expr::ENum { value } => format!("{value}"),
        Expr::EStr { value } => format!("{value:?}"),
        Expr::EBool { value } => format!("{value}"),
        Expr::ENil => "nil".to_string(),
        Expr::EBinary {
            left,
            operator,
            right,
        } => {
            format!(
                "{} {} {}",
                format_op(operator),
                format_expr(left),
                format_expr(right)
            )
        }
        Expr::EUnary { operator, right } => {
            format!("{} {}", format_op(operator), format_expr(right))
        }
        Expr::EGrouping { expression } => {
            format!("group {}", format_expr(expression))
        }
        Expr::EVariable { name } => format!("{name}"),
    }
}

// pub fn main() {
//     let expression = Expr::binary(
//         Expr::unary(Operator::OSub, Expr::num("123")),
//         Operator::OMul,
//         Expr::grouping(Expr::num("45.67")),
//     );
//     println!("{}", format_expr(&expression));
// }
