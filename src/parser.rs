// parser.rs

use crate::tokenizer::Tokens;

pub type AST = ();
pub type Error = ();

pub fn parse(tokens: Tokens) -> Result<AST, Error> {
    println!("Parsing");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn its_alive() {
        assert_eq!(true, true);
    }
}