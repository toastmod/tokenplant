use crate::tokenizer::{FunctionTokenParse, FunctionalToken, Token};

pub struct Search {
    thing_to_search: String
}

impl FunctionTokenParse for Search {
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

        Box::new(Search {
            thing_to_search: str
        })
    }
}

impl FunctionalToken for Search {
    fn functionality(&self, complete_stack: &[crate::tokenizer::Token], origin_tree: &mut crate::tokenizer::Tokenizer) -> Token {
        match origin_tree.char_map.get(&self.thing_to_search.chars().nth(0).unwrap()) {
            Some(r) => match r {
                crate::tokenizer::Relation::CharIsToken(_) => todo!(),
                crate::tokenizer::Relation::Tokens(_) => todo!(),
            },
            None => todo!(),
        };
    }
}