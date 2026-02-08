// evaluate.rs
//
// Run a Lox program

use crate::parser::AST;

pub type Output = ();

pub fn evaluate(ast: AST) -> Output {
    println!("Evaluating");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn its_alive() {
        assert_eq!(true, true);
    }
}