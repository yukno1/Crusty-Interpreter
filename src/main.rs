
// initially break the project into 4 parts
// Each in own file
mod reader;
mod tokenizer;
mod parser;
mod evaluate;


fn main() {
    println!("Hello, Lox!");
    reader::read_source();
    tokenizer::tokenize();
    parser::parse();
    evaluate::evaluate();
}
