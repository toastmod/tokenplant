use tokenplant::tokenizer::{self, FunctionTokenParse, Tokenizer};

mod mips;
use mips::*;

fn main() {
    let mut tokenizer = Tokenizer::<MIPSCCToken>::default();
    // tokenizer.add_def("print", Some(print::Print::parse));
    // stack.eval(&mut tokenizer);
    
    tokenizer.add_def("int", Some(DeclarationToken::parse));
    tokenizer.add_def("void", Some(DeclarationToken::parse));
    tokenizer.add_def("(", Some(Parenthesis::parse));
    // tokenizer.add_def(",", None);

    // let stack = tokenizer.tokenize("print \"hello world\"".as_bytes());
    let stack = tokenizer.tokenize("int hi (int lol)".as_bytes());
    stack.eval(&mut tokenizer);
    

}
