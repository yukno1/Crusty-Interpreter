// parser.rs

use std::usize;

use crate::ast::{AST, Expr, Operator};
use crate::tokenizer::{Token, TokenType, Tokens};
use TokenType::*;

impl From<&Token> for Operator {
    fn from(tok: &Token) -> Operator {
        match tok.toktype {
            TPlus => Operator::OAdd,
            TMinus => Operator::OSub,
            TStar => Operator::OMul,
            TSlash => Operator::ODiv,
            TLess => Operator::OLt,
            TLessEqual => Operator::OLe,
            TGreater => Operator::OGt,
            TGreaterEqual => Operator::OGe,
            TEqualEqual => Operator::OEq,
            TBangEqual => Operator::ONe,
            TAnd => Operator::OAdd,
            TOr => Operator::OOr,
            TBang => Operator::ONot,
            _ => panic!("Not an operator {:?}", tok.toktype),
        }
    }
}

#[derive(Debug)]
pub struct Error {}

struct Parser {
    tokens: Vec<Token>,
    n: usize,
}

impl Parser {
    fn new(tokens: Tokens) -> Parser {
        Parser {
            tokens: tokens.tokens,
            n: 0,
        }
    }

    fn accept(&mut self, toktype: TokenType) -> bool {
        if !self.at_end() && self.tokens[self.n].toktype == toktype {
            self.n += 1;
            true
        } else {
            false
        }
    }

    // accept any token from a list of possible types
    fn accepts<const N: usize>(&mut self, toktypes: [TokenType; N]) -> bool {
        if !self.at_end() && toktypes.contains(&self.tokens[self.n].toktype) {
            self.n += 1;
            true
        } else {
            false
        }
    }

    // return last matched token (a borrow)
    fn last_token(&mut self) -> &Token {
        &self.tokens[self.n - 1]
    }

    fn last_lexeme(&self) -> &String {
        &self.tokens[self.n - 1].lexeme
    }

    fn at_end(&self) -> bool {
        self.n >= self.tokens.len()
    }

    fn parse_top(&mut self) -> Result<AST, Error> {
        Ok(AST {
            top: Some(self.parse_expression()),
        })
    }

    fn parse_expression(&mut self) -> Expr {
        let left = self.parse_term();
        if self.accepts([TPlus, TMinus, TStar, TSlash]) {
            let operator = Operator::from(self.last_token());
            let right = self.parse_term();
            Expr::binary(left, operator, right)
        } else {
            left
        }
    }

    // parse a single value
    fn parse_term(&mut self) -> Expr {
        if self.accept(TokenType::TNumber) {
            Expr::num(self.last_lexeme())
        } else if self.accept(TokenType::TString) {
            Expr::str(self.last_lexeme())
        } else {
            panic!("Syntax Error!")
        }
    }
}

pub fn parse(tokens: Tokens) -> Result<AST, Error> {
    println!("Parsing");
    Parser::new(tokens).parse_top()
    // Ok(AST { top: None })
}

#[cfg(test)]
mod tests {
    use super::*;

    // helper
    fn parse_string(s: &str) -> AST {
        use crate::reader::Source;
        use crate::tokenizer::tokenize;
        let source = Source::from(s);
        let tokens = tokenize(source).unwrap();
        parse(tokens).unwrap()
    }

    #[test]
    fn its_alive() {
        assert_eq!(true, true);
    }

    #[test]
    fn test_term() {
        assert_eq!(
            parse_string("123"),
            AST {
                top: Some(Expr::num("123"))
            }
        );
    }

    #[test]
    fn test_binary() {
        assert_eq!(
            parse_string("1+2"),
            AST {
                top: Some(Expr::binary(Expr::num("1"), Operator::OAdd, Expr::num("2")))
            }
        );
    }
}
