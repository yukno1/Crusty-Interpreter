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
            _ => todo!(),
        }
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
        let scanner = Scanner::new(r"!!=<<=>>====");
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
}
