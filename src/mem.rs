use crate::tokenizer::{FunctionTokenParse, FunctionalToken, Token};

/// Remember the following quoted string with no functionality
pub struct Mem {
    thing_to_mem: String
}

impl FunctionTokenParse for Mem {
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

        Box::new(Mem {
            thing_to_mem: str
        })
    }
}

impl FunctionalToken for Mem {
    fn functionality(&self, complete_stack: &[crate::tokenizer::Token], my_index: usize, origin_tree: &mut crate::tokenizer::Tokenizer) -> Token {
        origin_tree.add_def(&self.thing_to_mem, None);
        Token::Blank
    }
}