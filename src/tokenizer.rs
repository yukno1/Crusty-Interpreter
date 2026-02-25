// tokenizer.rs
use crate::reader::Source;

#[derive(Debug, PartialEq)]
#[allow(dead_code)]
pub enum TokenType {
    // single-character tokens
    TLeftParen,
    TRightParen,
    TLeftBrace,
    TRightBrace,
    TComma,
    TDot,
    TMinus,
    TPlus,
    TSemiColon,
    TSlash,
    TStar,
    // one or two character tokens
    TBang,
    TBangEqual,
    TEqual,
    TEqualEqual,
    TGreater,
    TGreaterEqual,
    TLess,
    TLessEqual,
    // literals
    TIdentifier,
    TString,
    TNumber,
    // keywords
    TAnd,
    TClass,
    TElse,
    TFalse,
    TFun,
    TFor,
    TIf,
    TNil,
    TOr,
    TPrint,
    TReturn,
    TSuper,
    TThis,
    TTrue,
    TVar,
    TWhile,
    TEof,
}

#[derive(Debug, PartialEq)]
pub enum Literal {
    Str(String),
    Num(f64),
    None,
}

use TokenType::*;

#[derive(Debug, PartialEq)]
pub struct Token {
    pub toktype: TokenType,
    pub lexeme: String,
    // pub literal: Literal,
    pub line: usize,
}

impl Token {
    pub fn new(
        toktype: TokenType,
        lexeme: impl Into<String>,
        // literal: Literal,
        line: usize,
    ) -> Token {
        Token {
            toktype,
            lexeme: lexeme.into(),
            // literal,
            line,
        }
    }
}

#[derive(Debug)]
pub struct Tokens {
    pub tokens: Vec<Token>,
}

#[derive(Debug)]
enum ScanError {
    UnexpectedCharacter { line: usize, ch: char },
    UnterminatedString { line: usize },
}

#[derive(Debug)]
pub struct Error(Vec<ScanError>);

struct Scanner {
    source: Vec<char>,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
    errors: Vec<ScanError>,
}

impl Scanner {
    fn new(source: &str) -> Scanner {
        Scanner {
            source: source.chars().collect(),
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
            errors: Vec::new(),
        }
    }

    // collect errors in an internal vec
    // when scanning, return the error list as result
    // main program can decide what to do with errors
    fn error(&mut self, err: ScanError) {
        self.errors.push(err);
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    // top-level method that called to do everything
    fn scan_tokens(mut self) -> Result<Tokens, Error> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }
        self.tokens.push(Token::new(TEof, "", self.line));

        if self.errors.len() == 0 {
            Ok(Tokens {
                tokens: self.tokens,
            })
        } else {
            Err(Error(self.errors))
        }
    }

    fn advance(&mut self) -> char {
        let c = self.source[self.current];
        self.current += 1;
        c
    }

    fn matches(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        if self.source[self.current] != expected {
            return false;
        }
        self.current += 1;
        true
    }

    fn peek(&mut self) -> char {
        if self.is_at_end() {
            '\x00'
        } else {
            self.source[self.current]
        }
    }

    fn lexeme(&self) -> String {
        // return the current lexeme as a string
        // b/c store source as Vec<char> (not utf-8)
        self.source[self.start..self.current].iter().collect()
    }

    fn add_token(&mut self, toktype: TokenType) {
        self.tokens
            .push(Token::new(toktype, self.lexeme(), self.line));
    }

    fn add_token_with_literal(&mut self, toktype: TokenType, literal: Literal) {}

    fn scan_token(&mut self) {
        match self.advance() {
            '(' => self.add_token(TLeftParen),
            ')' => self.add_token(TRightParen),
            '{' => self.add_token(TLeftBrace),
            '}' => self.add_token(TRightBrace),
            ',' => self.add_token(TComma),
            '.' => self.add_token(TDot),
            '+' => self.add_token(TPlus),
            '-' => self.add_token(TMinus),
            '*' => self.add_token(TStar),
            ';' => self.add_token(TSemiColon),
            '/' => self.add_token(TSlash),
            '!' => {
                let toktype = if self.matches('=') { TBangEqual } else { TBang };
                self.add_token(toktype);
            }
            '=' => {
                let toktype = if self.matches('=') {
                    TEqualEqual
                } else {
                    TEqual
                };
                self.add_token(toktype);
            }
            '<' => {
                let toktype = if self.matches('=') { TLessEqual } else { TLess };
                self.add_token(toktype);
            }
            '>' => {
                let toktype = if self.matches('=') {
                    TGreaterEqual
                } else {
                    TGreater
                };
                self.add_token(toktype);
            }
            '/' => {
                if self.matches('/') {
                    // comment goes to end of line
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(TSlash);
                }
            }
            // ignore whitespace
            ' ' | '\r' | '\t' => {}
            '\n' => {
                self.line += 1;
            }
            '"' => self.string(),
            c if c.is_digit(10) => {
                self.number();
            }
            c if c.is_alphabetic() => {
                self.identifier();
            }
            e => {
                self.error(ScanError::UnexpectedCharacter {
                    line: self.line,
                    ch: e,
                });
            }
        }
    }

    fn string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }
        if self.is_at_end() {
            self.error(ScanError::UnterminatedString { line: self.line });
            return;
        }
        self.advance();
        // let value: String = self.source[self.start + 1..self.current - 1]
        //     .iter()
        //     .collect();
        // self.add_token_with_literal(TString, Literal::Str(value));
        self.add_token(TString);
    }

    fn number(&mut self) {
        while self.peek().is_digit(10) {
            self.advance();
        }
        if self.peek() == '.' {
            self.advance();
            while self.peek().is_digit(10) {
                self.advance();
            }
        }
        // let literal = Literal::Num(self.lexeme().parse().unwrap());
        self.add_token(TNumber);
    }

    fn identifier(&mut self) {
        while self.peek().is_alphabetic() || self.peek() == '_' || self.peek().is_numeric() {
            self.advance();
        }
        let toktype = match &self.lexeme()[..] {
            "and" => TAnd,
            "class" => TClass,
            "else" => TElse,
            "false" => TFalse,
            "for" => TFor,
            "fun" => TFun,
            "if" => TIf,
            "nil" => TNil,
            "or" => TOr,
            "print" => TPrint,
            "return" => TReturn,
            "super" => TSuper,
            "this" => TThis,
            "true" => TTrue,
            "var" => TVar,
            "while" => TWhile,
            _ => TIdentifier,
        };
        self.add_token(toktype);
    }
}

pub fn tokenize(source: Source) -> Result<Tokens, Error> {
    println!("Tokenizing");
    Scanner::new(&source.contents).scan_tokens()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn its_alive() {
        assert_eq!(true, true);
    }

    #[test]
    fn single_character() {
        let mut scanner = Scanner::new(r"(){},.+-;*");
        let tokens = scanner.scan_tokens();
        assert_eq!(
            tokens.unwrap().tokens,
            vec![
                Token::new(TLeftParen, "(".to_string(), 1),
                Token::new(TRightParen, ")".to_string(), 1),
                Token::new(TLeftBrace, "{".to_string(), 1),
                Token::new(TRightBrace, "}".to_string(), 1),
                Token::new(TComma, ",".to_string(), 1),
                Token::new(TDot, ".".to_string(), 1),
                Token::new(TPlus, "+".to_string(), 1),
                Token::new(TMinus, "-".to_string(), 1),
                Token::new(TSemiColon, ";".to_string(), 1),
                Token::new(TStar, "*".to_string(), 1),
                Token::new(TEof, "".to_string(), 1),
            ]
        )
    }

    #[test]
    fn two_character() {
        let scanner = Scanner::new(r"! != < <= > >= == =");
        let tokens = scanner.scan_tokens();
        assert_eq!(
            tokens.unwrap().tokens,
            vec![
                Token::new(TBang, "!".to_string(), 1),
                Token::new(TBangEqual, "!=".to_string(), 1),
                Token::new(TLess, "<".to_string(), 1),
                Token::new(TLessEqual, "<=".to_string(), 1),
                Token::new(TGreater, ">".to_string(), 1),
                Token::new(TGreaterEqual, ">=".to_string(), 1),
                Token::new(TEqualEqual, "==".to_string(), 1),
                Token::new(TEqual, "=".to_string(), 1),
                Token::new(TEof, "".to_string(), 1),
            ]
        )
    }

    #[test]
    fn strings() {
        let scanner = Scanner::new("\"Hello\" \"world!\"");
        let tokens = scanner.scan_tokens();
        assert_eq!(
            tokens.unwrap().tokens,
            vec![
                Token::new(TString, "\"Hello\"", 1),
                Token::new(TString, "\"world!\"", 1),
                Token::new(TEof, "".to_string(), 1),
            ]
        )
    }

    #[test]
    fn numbers() {
        let scanner = Scanner::new("12345 123.45");
        let tokens = scanner.scan_tokens();
        assert_eq!(
            tokens.unwrap().tokens,
            vec![
                Token::new(TNumber, "12345", 1),
                Token::new(TNumber, "123.45", 1),
                Token::new(TEof, "".to_string(), 1),
            ]
        )
    }

    #[test]
    fn identifiers() {
        let scanner = Scanner::new("abc abc123 ab_cd");
        let tokens = scanner.scan_tokens();
        assert_eq!(
            tokens.unwrap().tokens,
            vec![
                Token::new(TIdentifier, "abc", 1),
                Token::new(TIdentifier, "abc123", 1),
                Token::new(TIdentifier, "ab_cd", 1),
                Token::new(TEof, "".to_string(), 1),
            ]
        )
    }

    #[test]
    fn keywords() {
        let scanner = Scanner::new(
            "and class else false for fun if nil or print return super this true var while",
        );
        let tokens = scanner.scan_tokens();
        assert_eq!(
            tokens.unwrap().tokens,
            vec![
                Token::new(TAnd, "and", 1),
                Token::new(TClass, "class", 1),
                Token::new(TElse, "else", 1),
                Token::new(TFalse, "false", 1),
                Token::new(TFor, "for", 1),
                Token::new(TFun, "fun", 1),
                Token::new(TIf, "if", 1),
                Token::new(TNil, "nil", 1),
                Token::new(TOr, "or", 1),
                Token::new(TPrint, "print", 1),
                Token::new(TReturn, "return", 1),
                Token::new(TSuper, "super", 1),
                Token::new(TThis, "this", 1),
                Token::new(TTrue, "true", 1),
                Token::new(TVar, "var", 1),
                Token::new(TWhile, "while", 1),
                Token::new(TEof, "".to_string(), 1),
            ]
        )
    }
}
