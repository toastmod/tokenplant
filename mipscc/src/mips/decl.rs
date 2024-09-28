use tokenplant::tokenizer::StackRef;

use crate::{tokenizer::{skip_whitespace, FunctionTokenParse, FunctionalToken, Stack, Token, Tokenizer}, Declaration, MIPSCCToken, Operator};
use std::rc::Rc;

pub const INT: &str = "int";
pub const VOID: &str = "void";

#[derive(Debug, Clone)]
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

        let mut tmp_cursor = my_index+1; 
        match &complete_stack[tmp_cursor] {
            // Token::Char(_) => todo!(),
            // Token::ReturnCtx(_) => todo!(),
            Token::Func(ref f) => {
                match f.as_ctx() {
                    MIPSCCToken::Operator(o) => {
                        match o {
                            Operator::EQ => {
                                // expression stack = &[here..next_semicolon] 
                                let mut tmp_cur = tmp_cursor+1;
                                while tmp_cur < complete_stack.len() {
                                    if let Token::Char(c) = complete_stack[tmp_cur] {
                                        if c == ';' {
                                            return Token::ReturnCtx(MIPSCCToken::VarDecl {
                                                declaration: if let MIPSCCToken::Declaration(d) = self.as_ctx() {
                                                    d
                                                } else {
                                                    panic!("Fatal error occured")
                                                },
                                                expr: StackRef::Range(tmp_cursor+1..tmp_cur),
                                                
                                            });
                                        }
                                    }  
                                    tmp_cur += 1;
                                }
                                panic!("Reached unexpected EOF!")
                            },
                            _ => panic!("Unexpected operator!")
                        }
                    },

                    MIPSCCToken::Closure(param_closure) => {
                        match &complete_stack[tmp_cursor+1] {
                            Token::ReturnCtx(MIPSCCToken::Closure(body_closure)) => {
                                if let MIPSCCToken::Declaration(declaration) = self.as_ctx() {
                                    return Token::ReturnCtx(MIPSCCToken::FunctionDecl { declaration, param_closure, body_closure: body_closure.clone_ref() })
                                } else {
                                    panic!("Fatal error occured")
                                }
                            },

                            t => panic!("invalid token after parameter closure!: Token Type: {}", t.type_str())
                        }
                    }
                    _ => panic!("invalid operator after declaration!")
                }
            },
            _ => panic!("invalid token after delcaration!")
        }
    }
    
    fn as_ctx(&self) -> Self::ParserContext {
        MIPSCCToken::Declaration(Declaration {
            ty: self.ty,
            name: Rc::new(self.vname.clone()),
        })
    }
}