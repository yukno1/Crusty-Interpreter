// tokenizer.rs
use crate::reader::Source;

// express some high-level thinking about the problem of tokenizing.
// 1. tokenizing will return all of the tokens in some way
// 2. probabilty that tokenizing fails
// two type aliases are "stubs" for these possibilities

pub struct Tokens {}

#[derive(Debug)]
pub struct Error {}

// standard way of handling errors to return Result<T, E>
pub fn tokenize(source: &Source) -> Result<Tokens, Error> {
    println!("Tokenizing");
    Ok(Tokens {  })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn its_alive() {
        assert_eq!(true, true);
    }
}