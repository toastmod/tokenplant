use crate::{tokenizer::{skip_whitespace, FunctionTokenParse, FunctionalToken, Stack, Token, Tokenizer}, Declaration, MIPSCCToken};
use std::rc::Rc;

pub const INT: &str = "int";
pub const VOID: &str = "void";
pub struct DeclarationToken {
    ty: &'static str,
    vname: String,
}

impl FunctionTokenParse for DeclarationToken {
    type ParserContext = MIPSCCToken;
    fn parse(tokenizer: &Tokenizer<Self::ParserContext>, current_token: Token<Self::ParserContext>, stack: &[Token<Self::ParserContext>], next: &[u8], cursor: &mut usize) -> Box<dyn FunctionalToken<ParserContext = Self::ParserContext>> {
        let mut vname = String::new();

        // Parse alphabetical name
        println!("{}", cursor);
        'readvname: while next[*cursor].is_ascii_alphabetic() {

            println!("\t{}", next[*cursor] as char);
            vname.push(next[*cursor] as char);
            *cursor += 1;

            if *cursor >= next.len() {
                break 'readvname;
            }
            
        }

        // Move to next token
        skip_whitespace(next, cursor);

        Box::new(Self {
            ty: if let Token::Str(s) = current_token {
                match s.as_str() {
                    INT => INT,
                    VOID => VOID,
                    _ => panic!("Invalid typedef... what??!!")
                }
            } else {
                panic!("Invalid typedef... how?!")
            },
            vname,
        })
    }
}

impl FunctionalToken for DeclarationToken {
    type ParserContext = MIPSCCToken;
    fn postprocess(&self, complete_stack: &[Token<Self::ParserContext>], my_index: usize, origin_tree: &mut Tokenizer<Self::ParserContext>) -> Token<Self::ParserContext> {
        println!("Declaration!");
        Token::Blank
    }
    
    fn as_ctx(self) -> Self::ParserContext {
        MIPSCCToken::Declaration(Declaration {
            ty: self.ty,
            name: Rc::new(self.vname),
        })
    }
}