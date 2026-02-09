// reader.rs
//
// Read source code from a file

// a type-alias, set to "nothing" for now

#[derive(Debug)]
pub struct Source {
    pub contents: String,
}

#[derive(Debug)]
pub struct Error {}

pub fn read_source(filename: &str) -> Result<Source, Error> {
    println!("Reading source");
    let contents = std::fs::read_to_string(filename).unwrap();
    Ok(Source { contents })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn its_alive() {
        assert_eq!(true, true);
    }
}