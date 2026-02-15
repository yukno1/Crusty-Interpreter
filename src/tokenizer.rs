use std::collections::HashMap;

// tokenizer.rs
use crate::reader::Source;

#[derive(Debug, PartialEq)]
#[allow(dead_code)]
pub enum TokenType {
    // single-character tokens
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    SemiColon,
    Slash,
    Star,
    // one or two character tokens
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    // literals
    Identifier,
    String,
    Number,
    // keywords
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,
    Eof,
}

#[derive(Debug, PartialEq)]
pub enum Literal {
    Str(String),
    Num(f64),
    None,
}

#[derive(Debug, PartialEq)]
pub struct Token {
    pub toktype: TokenType,
    pub lexeme: String,
    pub literal: Literal,
    pub line: usize,
}

impl Token {
    pub fn new(
        toktype: TokenType,
        lexeme: impl Into<String>,
        literal: Literal,
        line: usize,
    ) -> Token {
        Token {
            toktype,
            lexeme: lexeme.into(),
            literal,
            line,
        }
    }
}

#[derive(Debug)]
pub struct Tokens {
    pub tokens: Vec<Token>,
}

#[derive(Debug)]
pub struct Error {}

struct Scanner {
    source: Vec<char>,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    fn new(source: &str) -> Scanner {
        Scanner {
            source: source.chars().collect(),
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
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
        self.tokens
            .push(Token::new(TokenType::Eof, "", Literal::None, self.line));

        Ok(Tokens {
            tokens: self.tokens,
        })
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

    fn add_token(&mut self, toktype: TokenType) {
        self.add_token_with_literal(toktype, Literal::None);
    }

    fn add_token_with_literal(&mut self, toktype: TokenType, literal: Literal) {
        let text: String = self.source[self.start..self.current].iter().collect();
        self.tokens
            .push(Token::new(toktype, text, literal, self.line));
    }

    fn scan_token(&mut self) {
        match self.advance() {
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            ',' => self.add_token(TokenType::Comma),
            '.' => self.add_token(TokenType::Dot),
            '+' => self.add_token(TokenType::Plus),
            '-' => self.add_token(TokenType::Minus),
            '*' => self.add_token(TokenType::Star),
            ';' => self.add_token(TokenType::SemiColon),
            '/' => self.add_token(TokenType::Slash),
            '!' => {
                let toktype = if self.matches('=') {
                    TokenType::BangEqual
                } else {
                    TokenType::Bang
                };
                self.add_token(toktype);
            }
            '=' => {
                let toktype = if self.matches('=') {
                    TokenType::EqualEqual
                } else {
                    TokenType::Equal
                };
                self.add_token(toktype);
            }
            '<' => {
                let toktype = if self.matches('=') {
                    TokenType::LessEqual
                } else {
                    TokenType::Less
                };
                self.add_token(toktype);
            }
            '>' => {
                let toktype = if self.matches('=') {
                    TokenType::GreaterEqual
                } else {
                    TokenType::Greater
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
                    self.add_token(TokenType::Slash);
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
                panic!("{e:?}");
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
            todo!("Unterminated string")
        }
        self.advance();
        let value: String = self.source[self.start + 1..self.current - 1]
            .iter()
            .collect();
        self.add_token_with_literal(TokenType::String, Literal::Str(value));
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
        let lexeme: String = self.source[self.start..self.current].iter().collect();
        let literal = Literal::Num(lexeme.parse().unwrap());
        self.add_token_with_literal(TokenType::Number, literal);
    }

    fn identifier(&mut self) {
        while self.peek().is_alphabetic() || self.peek() == '_' || self.peek().is_numeric() {
            self.advance();
        }
        // look for keyword TODO
        let lexeme: String = self.source[self.start..self.current].iter().collect();
        let toktype = match &lexeme[..] {
            "and" => TokenType::And,
            "class" => TokenType::Class,
            "else" => TokenType::Else,
            "false" => TokenType::False,
            "for" => TokenType::For,
            "fun" => TokenType::Fun,
            "if" => TokenType::If,
            "nil" => TokenType::Nil,
            "or" => TokenType::Or,
            "print" => TokenType::Print,
            "return" => TokenType::Return,
            "super" => TokenType::Super,
            "this" => TokenType::This,
            "true" => TokenType::True,
            "var" => TokenType::Var,
            "while" => TokenType::While,
            _ => TokenType::Identifier,
        };
        self.add_token(toktype);
    }
}

pub fn tokenize(_source: Source) -> Result<Tokens, Error> {
    println!("Tokenizing");
    let tokens = vec![];
    Ok(Tokens { tokens })
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
                Token::new(TokenType::LeftParen, "(".to_string(), Literal::None, 1),
                Token::new(TokenType::RightParen, ")".to_string(), Literal::None, 1),
                Token::new(TokenType::LeftBrace, "{".to_string(), Literal::None, 1),
                Token::new(TokenType::RightBrace, "}".to_string(), Literal::None, 1),
                Token::new(TokenType::Comma, ",".to_string(), Literal::None, 1),
                Token::new(TokenType::Dot, ".".to_string(), Literal::None, 1),
                Token::new(TokenType::Plus, "+".to_string(), Literal::None, 1),
                Token::new(TokenType::Minus, "-".to_string(), Literal::None, 1),
                Token::new(TokenType::SemiColon, ";".to_string(), Literal::None, 1),
                Token::new(TokenType::Star, "*".to_string(), Literal::None, 1),
                Token::new(TokenType::Eof, "".to_string(), Literal::None, 1),
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
                Token::new(TokenType::Bang, "!".to_string(), Literal::None, 1),
                Token::new(TokenType::BangEqual, "!=".to_string(), Literal::None, 1),
                Token::new(TokenType::Less, "<".to_string(), Literal::None, 1),
                Token::new(TokenType::LessEqual, "<=".to_string(), Literal::None, 1),
                Token::new(TokenType::Greater, ">".to_string(), Literal::None, 1),
                Token::new(TokenType::GreaterEqual, ">=".to_string(), Literal::None, 1),
                Token::new(TokenType::EqualEqual, "==".to_string(), Literal::None, 1),
                Token::new(TokenType::Equal, "=".to_string(), Literal::None, 1),
                Token::new(TokenType::Eof, "".to_string(), Literal::None, 1),
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
                Token::new(
                    TokenType::String,
                    "\"Hello\"",
                    Literal::Str("Hello".to_string()),
                    1
                ),
                Token::new(
                    TokenType::String,
                    "\"world!\"",
                    Literal::Str("world!".to_string()),
                    1
                ),
                Token::new(TokenType::Eof, "".to_string(), Literal::None, 1),
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
                Token::new(TokenType::Number, "12345", Literal::Num(12345.0), 1),
                Token::new(TokenType::Number, "123.45", Literal::Num(123.45), 1),
                Token::new(TokenType::Eof, "".to_string(), Literal::None, 1),
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
                Token::new(TokenType::Identifier, "abc", Literal::None, 1),
                Token::new(TokenType::Identifier, "abc123", Literal::None, 1),
                Token::new(TokenType::Identifier, "ab_cd", Literal::None, 1),
                Token::new(TokenType::Eof, "".to_string(), Literal::None, 1),
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
                Token::new(TokenType::And, "and", Literal::None, 1),
                Token::new(TokenType::Class, "class", Literal::None, 1),
                Token::new(TokenType::Else, "else", Literal::None, 1),
                Token::new(TokenType::False, "false", Literal::None, 1),
                Token::new(TokenType::For, "for", Literal::None, 1),
                Token::new(TokenType::Fun, "fun", Literal::None, 1),
                Token::new(TokenType::If, "if", Literal::None, 1),
                Token::new(TokenType::Nil, "nil", Literal::None, 1),
                Token::new(TokenType::Or, "or", Literal::None, 1),
                Token::new(TokenType::Print, "print", Literal::None, 1),
                Token::new(TokenType::Return, "return", Literal::None, 1),
                Token::new(TokenType::Super, "super", Literal::None, 1),
                Token::new(TokenType::This, "this", Literal::None, 1),
                Token::new(TokenType::True, "true", Literal::None, 1),
                Token::new(TokenType::Var, "var", Literal::None, 1),
                Token::new(TokenType::While, "while", Literal::None, 1),
                Token::new(TokenType::Eof, "".to_string(), Literal::None, 1),
            ]
        )
    }
}
