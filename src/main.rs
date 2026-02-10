use std::io::Write;

// initially break the project into 4 parts
// Each in own file
mod evaluator;
mod parser;
mod reader;
mod tokenizer;

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

fn run(source: reader::Source) -> Result<(), Error> {
    let tokens = tokenizer::tokenize(source)?;
    let ast = parser::parse(tokens)?;
    let _out = evaluator::evaluate(ast)?;
    Ok(())
}

fn run_file(filename: &str) -> Result<(), Error> {
    let source = reader::read_source(filename)?;
    run(source)
}

fn run_prompt() {
    // need to read from stdin
    // create a source object and pass to run
    let mut stdout = std::io::stdout();
    let mut stdin = std::io::stdin();
    loop {
        stdout.write_all(b"> ").unwrap();
        stdout.flush().unwrap();
        let mut buffer: String = String::new();
        stdin.read_line(&mut buffer).unwrap();
        let source = reader::Source { contents: buffer };
        match run(source) {
            Ok(_) => {}
            Err(e) => {
                println!("{e:?}");
            }
        }
    }
}

fn main() {
    println!("Hello, Lox!");

    let args: Vec<String> = std::env::args().collect();
    if args.len() == 1 {
        run_prompt();
    } else if args.len() == 2 {
        match run_file(&args[1]) {
            Ok(()) => {
                println!("Success!")
            }
            Err(e) => {
                eprintln!("Failure! {e:?}")
            }
        }
    } else {
        eprintln!("Usage: lox [filename]");
    }
}
