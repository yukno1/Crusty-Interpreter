// parser.rs

use crate::tokenizer::Tokens;

pub struct AST {}

#[derive(Debug)]
pub struct Error {}

pub fn parse(tokens: Tokens) -> Result<AST, Error> {
    println!("Parsing");
    Ok(AST {  })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn its_alive() {
        assert_eq!(true, true);
    }
}