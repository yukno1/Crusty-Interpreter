
// initially break the project into 4 parts
// Each in own file
mod reader;
mod tokenizer;
mod parser;
mod evaluate;

// mod only declares the existence of submodules, not importing code
// only need to specify mod in one place, usually in main

fn main() {
    println!("Hello, Lox!");
    let source = reader::read_source("somefile.lox").unwrap();
    let tokens = tokenizer::tokenize(&source).unwrap();
    let ast = parser::parse(tokens).unwrap();
    let out = evaluate::evaluate(ast).unwrap();
}
