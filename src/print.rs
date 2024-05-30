use crate::tokenizer::*;

pub struct Print {
    word_to_print: String,
}

impl FunctionTokenParse for Print {
    fn parse(current_token: Token, stack: &[Token], stack_position: usize, next: &[u8], cursor: &mut usize) -> Box<dyn FunctionalToken> { 

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

        Box::new(Print {
            word_to_print: str
        })

    }
}

impl FunctionalToken for Print {
    fn functionality(&self, complete_stack: &[Token]) {
        println!("{}", self.word_to_print);
    }
}


