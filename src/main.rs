use print::Print;
use tokenizer::{FunctionTokenParse, FunctionalToken, Token, Tokenizer};

mod tokenizer;

mod print;

fn main() {
    let mut tokenizer = Tokenizer::<()>::default();
    
    tokenizer.add_def("print", Some(Print::parse));

    let stack = tokenizer.tokenize("print \"hello world\"".as_bytes());
    stack.eval(&mut tokenizer);
    

}
