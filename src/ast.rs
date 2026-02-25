use crate::tokenizer::Token;

pub enum Expr {
    // Literal {
    //     value: tokenizer::Literal,
    // },
    Num {
        value: f64,
    },
    Str {
        value: String,
    },
    Bool {
        value: bool,
    },
    Nil,
    Binary {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    Unary {
        operator: Token,
        right: Box<Expr>,
    },
    Grouping {
        expression: Box<Expr>,
    },
}

impl Expr {
    fn num(value: f64) -> Expr {
        Expr::Num { value }
    }
    fn str(value: impl Into<String>) -> Expr {
        Expr::Str {
            value: value.into(),
        }
    }
    fn bool(value: bool) -> Expr {
        Expr::Bool { value }
    }
    fn nil() -> Expr {
        Expr::Nil
    }

    fn binary(left: Expr, operator: Token, right: Expr) -> Expr {
        Expr::Binary {
            left: left.into(),
            operator,
            right: right.into(),
        }
        // into() put value into a Box
    }

    fn unary(operator: Token, right: Expr) -> Expr {
        Expr::Unary {
            operator,
            right: right.into(),
        }
        // into() put value into a Box
    }

    fn grouping(expression: Expr) -> Expr {
        Expr::Grouping {
            expression: expression.into(),
        }
    }
}

pub fn format_expr(e: &Expr) -> String {
    match e {
        Expr::Num { value } => format!("{value}"),
        Expr::Str { value } => format!("{value:?}"),
        Expr::Bool { value } => format!("{value}"),
        Expr::Nil => "nil".to_string(),
        Expr::Binary {
            left,
            operator,
            right,
        } => {
            format!(
                "{} {} {}",
                operator.lexeme,
                format_expr(left),
                format_expr(right)
            )
        }
        Expr::Unary { operator, right } => {
            format!("{} {}", operator.lexeme, format_expr(right))
        }
        Expr::Grouping { expression } => {
            format!("group {}", format_expr(expression))
        }
    }
}

pub fn main() {
    use crate::tokenizer::{Literal, Token, TokenType};
    let expression = Expr::binary(
        Expr::unary(Token::new(TokenType::TMinus, "-", 1), Expr::num(123.0)),
        Token::new(TokenType::TStar, "*", 1),
        Expr::grouping(Expr::num(45.67)),
    );
    println!("{}", format_expr(&expression));
}
