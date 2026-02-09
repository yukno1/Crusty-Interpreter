
// initially break the project into 4 parts
// Each in own file
mod reader;
mod tokenizer;
mod parser;
mod evaluator;

// mod only declares the existence of submodules, not importing code
// only need to specify mod in one place, usually in main

// top-level error type captures every possible bad things 
// that can happen all in one place.
// manage error flow
#[derive(Debug)]
pub enum Error {
    Reader(reader::Error),
    Tokenizer(tokenizer::Error),
    Parser(parser::Error),
    Evaluator(evaluator::Error),
}

// conversion from low-level errors to the top-level error
// needed to make '?' work
impl From<reader::Error> for Error {
    fn from(e: reader::Error) -> Error {
        Error::Reader(e)
    }
}

impl From<tokenizer::Error> for Error {
    fn from(e: tokenizer::Error) -> Error {
        Error::Tokenizer(e)
    }
}

impl From<parser::Error> for Error {
    fn from(e: parser::Error) -> Error {
        Error::Parser(e)
    }
}

impl From<evaluator::Error> for Error {
    fn from(e: evaluator::Error) -> Error {
        Error::Evaluator(e)
    }
}


fn run() -> Result<(), Error> {
    let source = reader::read_source("somefile.lox")?;
    let tokens = tokenizer::tokenize(source)?;
    let ast = parser::parse(tokens)?;
    let out = evaluator::evaluate(ast)?;
    Ok(())
}

fn main() {
    println!("Hello, Lox!");
    match run() {
        Ok(()) => { println!("Success!") },
        Err(e) => { println!("Failure! {e:?}") },
    }
}
