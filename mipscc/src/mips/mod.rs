mod parenthesis;
mod decl;
use std::{default, rc::Rc};

pub use parenthesis::*;
pub use decl::*;
use tokenplant::tokenizer::StackRef;

use crate::tokenizer::{Stack, Token, Tokenizer};


#[derive(Default)]
pub enum Operator {
    #[default]
    ADD,
    SUB,
    EQ
}


#[derive(Default)]
pub struct Closure {
    pub body: Rc<Stack<MIPSCCToken>>
}

impl Closure {
    pub fn clone_ref(&self) -> Self {
        Self {
            body: Rc::clone(&self.body)
        }
    }
}

#[derive(Default)]
pub struct Declaration {
    pub ty: &'static str,
    pub name: Rc<String>
}

impl Declaration {
    pub fn clone_ref(&self) -> Self {
        Self {
            ty: self.ty,
            name: Rc::clone(&self.name),
        }
    }
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
        expr: StackRef
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