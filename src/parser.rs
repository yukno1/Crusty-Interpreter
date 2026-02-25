// parser.rs

use crate::ast::{AST, Expr};
use crate::tokenizer::{Token, TokenType, Tokens};

#[derive(Debug)]
pub struct Error {}

struct Parser {
    tokens: Vec<Token>,
    n: usize,
}

impl Parser {
    fn accept(&mut self, toktype: TokenType) -> bool {
        if !self.at_end() && self.tokens[self.n].toktype == toktype {
            self.n += 1;
            true
        } else {
            false
        }
    }

    // return last matched token (a borrow)
    fn last(&mut self) -> &Token {
        &self.tokens[self.n - 1]
    }

    fn at_end(&self) -> bool {
        self.n >= self.tokens.len()
    }
}

pub fn parse(_tokens: Tokens) -> Result<AST, Error> {
    println!("Parsing");
    Ok(AST { top: None })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn its_alive() {
        assert_eq!(true, true);
    }
}
