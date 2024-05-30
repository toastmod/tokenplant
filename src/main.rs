use tokenizer::{FunctionTokenParse, FunctionalToken, Token, Tokenizer};

mod tokenizer;
mod print;

use print::*;

fn main() {
    println!("Hello, world!");
    let mut tokenizer = Tokenizer::default();
    tokenizer.add_def("print", Some(Print::parse));

    tokenizer.tokenize("print \"hello world\"").execute();

}
