// reader.rs
//
// Read source code from a file

// a type-alias, set to "nothing" for now

pub type Source = ();

pub fn read_source(filename: &str) -> Source {
    println!("Reading source");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn its_alive() {
        assert_eq!(true, true);
    }
}