use std::{collections::{HashMap, HashSet}, rc::Rc};

/// Rules:
/// - One char tokens don't need space separaton on either side
/// - String tokens must end with a space or a char token
/// - Numbers cannot be tokens and will be evaluated into an expression
/// - Input must be ascii

pub type TokenBehaviour = Option<fn(Token, &[Token], usize, &[u8], &mut usize) -> Box<dyn FunctionalToken>>;

pub trait FunctionalToken {
    fn functionality(&self, complete_stack: &[Token]);
}

pub trait FunctionTokenParse {
    fn parse(current_token: Token, stack: &[Token], stack_position: usize, next: &[u8], cursor: &mut usize) -> Box<dyn FunctionalToken>;
}

pub enum Token {
    Str(Rc<String>),
    Char(char),
    Func(Box<dyn FunctionalToken>)
}

pub enum Relation {
    CharIsToken(TokenBehaviour),
    Tokens(HashMap<Rc<String>, TokenBehaviour>),
}

pub struct Stack {
    stack: Vec<Token>
}

impl Stack {
    pub fn execute(&self) {
        for token in &self.stack {
            if let Token::Func(functoken) = token {
                functoken.functionality(&self.stack)
            }
        }
    }
}

#[derive(Default)]
pub struct Tokenizer {
    char_map: HashMap<char, Relation>,
}

impl Tokenizer {

    pub fn tokenize(&mut self, encoded: &str) -> Stack {
        let mut cursor = 0usize;
        let src = encoded.as_bytes();
        let mut result_tokens = vec![];

        while cursor != encoded.len() {
            let cur_c = src[cursor] as char;


            // if s(src[cursor] as char)
            if let Some(r) = self.char_map.get(&cur_c) {
                match r {
                    Relation::CharIsToken(behaviour) => {
                        if let Some(f) = behaviour {
                            result_tokens.push(Token::Func((f)(Token::Char(cur_c), &result_tokens, result_tokens.len()-1, src, &mut cursor)));
                        }else{
                            result_tokens.push(Token::Char(cur_c));
                        }
                    },
                    Relation::Tokens(tokens) => {
                        for (token, _) in tokens {
                            let tmp_cursor = cursor + token.len();
                            let mut tmp_addchar = None;
                            if (
                                // String Tokens must either end with a space...
                                (src[tmp_cursor] as char).is_ascii_whitespace() ||
                                // Or with a single char string
                                ({
                                    match self.char_map.get(&(src[tmp_cursor] as char)) {
                                        // If so, add it and treat it as a space
                                        Some(r1) => if let Relation::CharIsToken(behaviour) = r1 {
                                            tmp_addchar = Some((Token::Char(cur_c), behaviour));
                                            true
                                        } else {
                                            false
                                        },
                                        None => false,
                                    }
                                })
                            )
                            && (&src[cursor..tmp_cursor] == token.as_bytes()) {
                                // Token(s) found!
                                if let Some((t, beh)) = tmp_addchar {
                                    if let Some(f) = beh {
                                        result_tokens.push(Token::Func((f)(t, &result_tokens, result_tokens.len()-1, src, &mut cursor)));
                                    }else{
                                        result_tokens.push(t);
                                    }
                                    // Skip the parsed char
                                    cursor = tmp_cursor+1;
                                } else {
                                    // Move cursor to next char
                                    cursor = tmp_cursor;
                                }

                                // Skip whitespace
                                while (src[cursor] as char).is_ascii_whitespace() {
                                    cursor += 1;
                                }

                                result_tokens.push(Token::Str(Rc::clone(&token)));
                                continue;

                            }else{
                                // Not match
                                continue;
                            }
                        }

                        // If fallen here, Token could not be matched at all


                    },
                };
            } else {
                // Unknown char token
            }
        }

        Stack {
            stack: result_tokens
        }

    }

    pub fn add_def(&mut self, token: &str, behaviour: TokenBehaviour) {
        let c = token.chars().nth(0).unwrap();

        if token.len() == 1 {
            self.char_map.insert(c, Relation::CharIsToken(behaviour));
            return;
        }

        let token = Rc::new(String::from(token));

        match self.char_map.get_mut(&c) {
            Some(r) => {
                if let Relation::Tokens(cset) = r {
                    cset.insert(token, behaviour);
                }
            },
            None => {
                let mut cset = HashMap::new();
                cset.insert(Rc::clone(&token), behaviour);
                self.char_map.insert(c, Relation::Tokens(cset));
            },
        };

    }

    pub fn add_parser<P: FunctionTokenParse>(&mut self, token: &str) {
        self.add_def(token, Some(P::parse))
    }

}