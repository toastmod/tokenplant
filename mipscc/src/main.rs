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
    tokenizer.add_def(";", None);
    // tokenizer.add_def(",", None);

    // let stack = tokenizer.tokenize("print \"hello world\"".as_bytes());
    let stack = tokenizer.tokenize("int hi (int lol = 5;) { } ".as_bytes());
    stack.eval(&mut tokenizer);
    stack.print_content(&|c| {
        match c {
            MIPSCCToken::Blank => String::from("[MIPSBlank] "),
            MIPSCCToken::Closure(_) => String::from("Closure "),
            MIPSCCToken::Declaration(_) => String::from("UnprocessedDecl "),
            MIPSCCToken::Operator(_) => String::from("Op "),
            MIPSCCToken::VarDecl { declaration, expr } => format!("Var[[{}] {}] ", declaration.ty, declaration.name),
            MIPSCCToken::FunctionDecl { declaration, param_closure, body_closure } => format!("Func[[{}] {}] ",declaration.ty, declaration.name),
            MIPSCCToken::Downcast(_) => String::from("DowncastReq "),
        }
    })
    

}
