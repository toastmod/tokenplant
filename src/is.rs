use crate::{mem::Mem, tokenizer::{FunctionTokenParse, FunctionalToken, Token, Tokenizer}};

pub struct Is {
    before: usize,
    after: String
}

impl FunctionTokenParse for Is {
    fn parse(current_token: crate::tokenizer::Token, stack: &[crate::tokenizer::Token], next: &[u8], cursor: &mut usize) -> Box<dyn crate::tokenizer::FunctionalToken> {

        let p1 = cursor.clone();
        let mut p2 = p1+1;
        if next[p1] as char == '"' {
            while next[p2] as char != '"' {
                p2 += 1;
            }
        }

        let str_bytes = &next[p1 .. p2];      

        let mut str = String::new();

        for c in str_bytes {
            str.push(c.clone() as char);
        }

        Box::new(Is {
            before: stack.len()-1,
            after: str 
        })
    }
}

impl FunctionalToken for Is {
    fn functionality(&self, complete_stack: &[Token], origin_tree: &mut Tokenizer) -> Token {
        origin_tree.add_def(complete_stack[self.before].to_str(), Some(Mem::parse)); 
        Token::Blank
    }
}