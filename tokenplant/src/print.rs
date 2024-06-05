use crate::tokenizer::*;

pub struct Print; 

impl FunctionTokenParse for Print {

    type ParserContext = ();

    fn parse(tokenizer: &Tokenizer<Self::ParserContext>, current_token: Token<Self::ParserContext>, stack: &[Token<Self::ParserContext>], next: &[u8], cursor: &mut usize) -> Box<dyn FunctionalToken<ParserContext = Self::ParserContext>> {
        // let p1 = cursor.clone();
        // let mut p2 = p1+1;
        // if next[p1] as char == '"' {
        //     while next[p2] as char != '"' {
        //         p2 += 1;
        //     }
        // }

        // let str_bytes = &next[p1+1 .. p2];      

        // let mut str = String::new();

        // for c in str_bytes {
        //     str.push(c.clone() as char);
        // }

        // Box::new(Print {
        //     word_to_print: str
        // })

        Box::new(Print)
    }

    
}

impl FunctionalToken for Print {

    type ParserContext = ();

    fn postprocess(&self, complete_stack: &[Token<Self::ParserContext>], my_index: usize, origin_tree: &mut Tokenizer<Self::ParserContext>) -> Token<Self::ParserContext> {
        if let Token::Str(s) = &complete_stack[my_index+1] {
            println!("{}", s);
        }

        Token::Blank
    }

    fn as_ctx(self) -> Self::ParserContext {
        () 
    }
}


