mod parenthesis;
mod decl;
use std::{default, rc::Rc};

pub use parenthesis::*;
pub use decl::*;

use crate::tokenizer::{Stack, Token, Tokenizer};


#[derive(Default)]
pub enum Operator {
    #[default]
    ADD,
    SUB
}


#[derive(Default)]
pub struct Closure {
    pub body: Stack<MIPSCCToken>
}

#[derive(Default)]
pub struct Declaration {
    pub ty: &'static str,
    pub name: Rc<String>
}

#[derive(Default)]
pub enum MIPSCCToken {

    #[default]
    Blank,

    /// An enclosure or parenthesis holding multiple MIPSCC Tokens
    Closure(Closure), 

    // A named declaration with a type attribute
    Declaration(Declaration),

    /// A one character operator
    Operator(Operator), 


    // === Second level tokens ===

    /// A variable declaration.
    VarDecl {
        declaration: Declaration,
        expr: Stack<MIPSCCToken> 
    },

    /// A function consisting of a declaration and two closures.
    FunctionDecl {
        declaration: Declaration,
        param_closure: Closure,
        body_closure: Closure
    },

    /// Downcast to a regular token with no context.
    Downcast(Token<()>)

}