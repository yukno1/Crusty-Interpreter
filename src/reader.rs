// reader.rs
//
// Read source code from a file

// a type-alias, set to "nothing" for now

pub struct Source {}

#[derive(Debug)]
pub struct Error {}

pub fn read_source(filename: &str) -> Result<Source, Error> {
    println!("Reading source");
    Ok(Source {  })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn its_alive() {
        assert_eq!(true, true);
    }
}