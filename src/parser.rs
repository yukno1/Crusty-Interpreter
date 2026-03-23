// parser.rs

use std::usize;

use crate::ast::{AST, Expr, Operator, Stmt};
use crate::tokenizer::{Token, TokenType, Tokens};
use TokenType::*;

// map from token to operator
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
pub enum Error {
    SyntaxError { line: usize, msg: String },
}

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

    fn consume(&mut self, toktype: TokenType, msg: &str) -> Result<(), Error> {
        // require next token to exactly match given token or else error
        if !self.accept(toktype) {
            Err(self.syntax_error(msg))
        } else {
            Ok(())
        }
    }

    // helper function
    fn syntax_error(&self, msg: &str) -> Error {
        Error::SyntaxError {
            line: self.tokens[self.n].line,
            msg: format!("{msg} at {:?}", self.tokens[self.n].lexeme),
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
        self.n >= self.tokens.len() || self.tokens[self.n].toktype == TEof
    }

    fn parse_top(&mut self) -> Result<AST, Error> {
        let top = self.parse_statements()?;
        if !self.at_end() {
            return Err(self.syntax_error("Unparsed input"));
        }
        Ok(AST { top })
    }

    // for statements, need more methods
    fn parse_statements(&mut self) -> Result<Vec<Stmt>, Error> {
        // parse 0 or more statements
        let mut statements = Vec::new();
        while !self.at_end() {
            statements.push(self.parse_declration()?);
        }
        Ok(statements)
    }

    fn parse_declration(&mut self) -> Result<Stmt, Error> {
        // parse a decl or stmt
        // there are tricky rules about where "var" can go in lox
        // so var decl is singled out as special case
        if self.accept(TVar) {
            self.parse_var_declaration()
        } else {
            self.parse_statement()
        }
    }

    fn parse_statement(&mut self) -> Result<Stmt, Error> {
        // parse a single statement
        if self.accept(TPrint) {
            self.parse_print_statement()
        } else {
            self.parse_expression_statement()
        }
    }

    fn parse_print_statement(&mut self) -> Result<Stmt, Error> {
        // print expression
        let value = self.parse_expression()?;
        self.consume(TSemiColon, "Expect ';' after value.")?;
        Ok(Stmt::print(value))
    }

    fn parse_expression_statement(&mut self) -> Result<Stmt, Error> {
        let value = self.parse_expression()?;
        self.consume(TSemiColon, "Expect ';' after value.")?;
        Ok(Stmt::expression(value))
    }

    pub fn parse_var_declaration(&mut self) -> Result<Stmt, Error> {
        self.consume(TIdentifier, "Expect variable name")?;
        let name = self.last_lexeme().clone();
        let mut initializer = None;
        if self.accept(TEqual) {
            initializer = Some(self.parse_expression()?);
        }
        self.consume(TSemiColon, "Expect ';' after variable declaration.")?;
        Ok(Stmt::var_decl(name, initializer))
    }

    pub fn parse_expression(&mut self) -> Result<Expr, Error> {
        let left = self.parse_unary()?;
        if self.accepts([
            TPlus,
            TMinus,
            TStar,
            TSlash,
            TLess,
            TLessEqual,
            TGreater,
            TGreaterEqual,
            TEqualEqual,
            TBangEqual,
        ]) {
            let operator = Operator::from(self.last_token());
            let right = self.parse_unary()?;
            Ok(Expr::binary(left, operator, right))
        } else {
            Ok(left)
        }
    }

    fn parse_unary(&mut self) -> Result<Expr, Error> {
        if self.accepts([TMinus, TBang]) {
            let op = Operator::from(self.last_token());
            Ok(Expr::unary(op, self.parse_unary()?))
        } else {
            self.parse_primary()
        }
    }

    // parse a single value
    fn parse_primary(&mut self) -> Result<Expr, Error> {
        Ok(if self.accept(TNumber) {
            Expr::num(self.last_lexeme())
        } else if self.accept(TString) {
            Expr::str(self.last_lexeme())
        } else if self.accept(TNil) {
            Expr::nil()
        } else if self.accept(TTrue) {
            Expr::bool(true)
        } else if self.accept(TFalse) {
            Expr::bool(false)
        } else if self.accept(TLeftParen) {
            let expr = self.parse_expression()?;
            self.consume(TRightParen, "Expected ')' after expression")?;
            Expr::grouping(expr)
        } else if self.accept(TIdentifier) {
            Expr::variable(self.last_lexeme())
        } else {
            return Err(self.syntax_error("Expected primary"));
        })
    }
}

pub fn parse(tokens: Tokens) -> Result<AST, Error> {
    // println!("Parsing");
    Parser::new(tokens).parse_top()
    // Ok(AST { top: None })
}

#[cfg(test)]
mod tests {
    use super::*;

    // helper
    // fn parse_string(s: &str) -> AST {
    //     use crate::reader::Source;
    //     use crate::tokenizer::tokenize;
    //     let source = Source::from(s);
    //     let tokens = tokenize(source).unwrap();
    //     parse(tokens).unwrap()
    // }

    fn parse_expr_string(s: &str) -> Expr {
        use crate::reader::Source;
        use crate::tokenizer::tokenize;
        let source = Source::from(s);
        let tokens = tokenize(source).unwrap();
        Parser::new(tokens).parse_expression().unwrap()
    }

    #[test]
    fn its_alive() {
        assert_eq!(true, true);
    }

    #[test]
    fn test_primary() {
        assert_eq!(parse_expr_string("123"), Expr::num("123"));
        assert_eq!(parse_expr_string("\"hello\""), Expr::str("\"hello\""));
        assert_eq!(parse_expr_string("nil"), Expr::nil());
        assert_eq!(parse_expr_string("true"), Expr::bool(true));
        assert_eq!(parse_expr_string("false"), Expr::bool(false));
        assert_eq!(parse_expr_string("(2)"), Expr::grouping(Expr::num("2")));
    }

    #[test]
    fn test_binary() {
        assert_eq!(
            parse_expr_string("1+2"),
            Expr::binary(Expr::num("1"), Operator::OAdd, Expr::num("2"))
        );
    }
}
