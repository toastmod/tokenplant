use tokenizer::{FunctionTokenParse, FunctionalToken, Token, Tokenizer};

pub mod tokenizer;
pub mod print;

pub fn is_alpha(c: u8) -> bool {
    ((c >= 97) && (c <= 122)) || ((c >= 64) && (c <= 90))
}