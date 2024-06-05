use crate::{tokenizer::{skip_whitespace, FunctionTokenParse, FunctionalToken, Stack, Token, Tokenizer}, Closure, MIPSCCToken};

pub struct Parenthesis {
    inner_stack: Stack<MIPSCCToken>
}

impl FunctionTokenParse for Parenthesis {
    type ParserContext = MIPSCCToken;
    fn parse(tokenizer: &Tokenizer<Self::ParserContext>, current_token: Token<Self::ParserContext>, stack: &[Token<Self::ParserContext>], next: &[u8], cursor: &mut usize) -> Box<dyn FunctionalToken<ParserContext = Self::ParserContext>> {


        println!("{}", cursor);

        let start_parenthesis = match current_token {
            Token::Char(c) => c,
            _ => panic!("Fatal! Invalid token!")
        };
        let end_parenthesis = match start_parenthesis {
            '(' => ')',
            '{' => '}',
            _ => panic!("Invalid parenthesis!")
        };

        let c = cursor.clone();

        while *cursor < next.len() {
            if next[*cursor] as char == end_parenthesis {
                return Box::new(Self {
                    inner_stack: tokenizer.tokenize(&next[c..*cursor]),
                })
            } else {
                *cursor += 1;
            }
        }

        // Move to next token
        *cursor += 1;
        skip_whitespace(next, cursor);

        panic!("Unmatched parenthesis!")
    }
}

impl FunctionalToken for Parenthesis {
    type ParserContext = MIPSCCToken;
    fn postprocess(&self, complete_stack: &[Token<Self::ParserContext>], my_index: usize, origin_tree: &mut Tokenizer<Self::ParserContext>) -> Token<Self::ParserContext> {
        println!("== Parenthesis ==");
        self.inner_stack.eval(origin_tree);
        println!("=================");
        Token::Blank
    }

    fn as_ctx(self) -> Self::ParserContext {
        MIPSCCToken::Closure(Closure {
            body: self.inner_stack,
        }) 
    }
}