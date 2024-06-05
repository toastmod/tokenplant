use std::{collections::{HashMap, HashSet}, marker::PhantomData, rc::Rc};

/// Rules:
/// - One char tokens don't need space separaton on either side
/// - String tokens must end with a space or a char token
/// - Numbers cannot be tokens and will be evaluated into an expression
/// - Input must be ascii
/// - One char tokens must be symbols, cannot be alphanumeric

/// If cursor is on a whitespace, skips to the next non-whitespace character
pub fn skip_whitespace(next: &[u8], cursor: &mut usize) {
    if *cursor >= next.len() {
        return;
    }
    // Move to next token
    while (next[*cursor] as char).is_ascii_whitespace() {
        *cursor += 1;

        if *cursor >= next.len() {
            break;
        }
    } 
}

/// If cursor is on a character, skips to the next single char character or whitespace character
pub fn skip_word<Ctx>(next: &[u8], char_map: &HashMap<char, Relation<Ctx>>, cursor: &mut usize ) {

    if *cursor >= next.len() {
        return;
    }

    // Move to next whitespace 
    'findWsOrCharTok: while !(next[*cursor] as char).is_ascii_whitespace() {

        let cur_c = next[*cursor] as char;
        if let Some(r) = char_map.get(&cur_c) {
            match r {
                Relation::CharIsToken(_) => break 'findWsOrCharTok,
                _ => ()
            };
        }

        *cursor += 1;
        if *cursor >= next.len() {
            break;
        }
    } 
}

pub type TokenBehaviour<Ctx> = Option<fn(&Tokenizer<Ctx>, Token<Ctx>, &[Token<Ctx>], &[u8], &mut usize) -> Box<dyn FunctionalToken<ParserContext = Ctx>>>;

/// For use of referencing when postprocessing.
pub type TokenHandle = usize;

pub trait FunctionalToken {
    type ParserContext;
    fn postprocess(&self, complete_stack: &[Token<Self::ParserContext>], my_index: usize, origin_tree: &mut Tokenizer<Self::ParserContext>) -> Token<Self::ParserContext>;
    fn as_ctx(self) -> Self::ParserContext;
}

pub trait FunctionTokenParse {
    type ParserContext;
    fn parse(tokenizer: &Tokenizer<Self::ParserContext>, current_token: Token<Self::ParserContext>, stack: &[Token<Self::ParserContext>], next: &[u8], cursor: &mut usize) -> Box<dyn FunctionalToken<ParserContext = Self::ParserContext>>;
}

pub enum PostProcResult<Ctx> {
    Same,
    Drop,
    NewToken(Token<Ctx>)
}

pub enum Token<Ctx> {
    // Token evaluates to a string
    Str(Rc<String>),

    // Token evaluates to a character
    Char(char),

    // // Token evaluates to a number
    // Number(Number),

    // Token evaluates to a functional token
    Func(Box<dyn FunctionalToken<ParserContext = Ctx>>),

    // Token is blank and can be discarded
    Blank
}

impl<T> Token<T> {
    pub fn to_str(&self) -> &str {
        if let Token::Str(s) = self {
            s
        } else if let Token::Char(c) = self {
            ""
        }else{
            ""
        }
    }
}

pub enum Relation<Ctx> {
    CharIsToken(TokenBehaviour<Ctx>),
    Tokens(HashMap<Rc<String>, TokenBehaviour<Ctx>>),
}


#[derive(Default)]
pub struct Stack<Ctx> {
    stack: Vec<Token<Ctx>>
}

impl<Ctx> Stack<Ctx> {
    pub fn eval(&self, origin_tree: &mut Tokenizer<Ctx>) {
        for (index, token) in self.stack.iter().enumerate() {
            if let Token::Func(functoken) = token {
                functoken.postprocess(&self.stack, index, origin_tree);
            }
        }
    }
}

#[derive(Default)]
pub struct Tokenizer<Ctx> {
    pub char_map: HashMap<char, Relation<Ctx>>,
    _mark: PhantomData<Ctx>
}

impl<Ctx> Tokenizer<Ctx> {

    pub fn tokenize(&self, src: &[u8]) -> Stack<Ctx> {
        let mut cursor = 0usize;
        let mut result_tokens = vec![];

        'searchmap: while cursor != src.len() {
            let cur_c = src[cursor] as char;

            if let Some(r) = self.char_map.get(&cur_c) {
                match r {
                    Relation::CharIsToken(behaviour) => {
                        cursor += 1;
                        // Skip whitespace
                        while (src[cursor] as char).is_ascii_whitespace() {
                            cursor += 1;
                        }
                        if let Some(f) = behaviour {
                            result_tokens.push(Token::Func((f)(self, Token::Char(cur_c), &result_tokens, src, &mut cursor)));
                        }else{
                            result_tokens.push(Token::Char(cur_c));
                        }
                    },
                    Relation::Tokens(tokens) => {
                        'searchrelations: for (token, behaviour) in tokens {
                            let tmp_cursor = cursor + token.len();
                            let mut tmp_addchar = None;
                            if (
                                // String Tokens must either end with a space...
                                (src[tmp_cursor] as char).is_ascii_whitespace() ||
                                // Or with a single char string
                                ({
                                    match self.char_map.get(&(src[tmp_cursor] as char)) {
                                        // If the ending char is a single char token, queue it up for tokenization ahead of time
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
                                // Token found!

                                // If the excluded ending character is non-whitespace but there was no token collected, 
                                // then we didn't actually find the right token yet. 
                                // Search the next relation.
                                if (!(src[tmp_cursor] as char).is_ascii_whitespace()) && (tmp_addchar.is_none()) {
                                    continue 'searchrelations;
                                }

                                // Make sure cursor is moved before functional token parse call
                                // This will land the cursor either on a whitespace or the immediate char token
                                cursor = tmp_cursor;

                                // Skip whitespace, this will do nothing if already on the immediate char token
                                skip_whitespace(src, &mut cursor);

                                // Handle first token, with the cursor either on the whitespace or immediate char token
                                let new_token = Token::Str(Rc::clone(&token));
                                if let Some(f) = behaviour {
                                    result_tokens.push(Token::Func((f)(self, new_token, &result_tokens, src, &mut cursor)));
                                }else{
                                    result_tokens.push(new_token);
                                }

                                // Handle immediate char token if there was one
                                if let Some((t, beh)) = tmp_addchar {
                                    
                                    // Move to next front char
                                    cursor += 1;
                                    skip_whitespace(src, &mut cursor);

                                    if let Some(f) = beh {
                                        result_tokens.push(Token::Func((f)(self, t, &result_tokens, src, &mut cursor)));
                                    }else{
                                        result_tokens.push(t);
                                    }
                                }

                                // Relation was found and recorded, 
                                // Cursor should be on next word, move on.
                                continue 'searchmap;

                            }else{
                                // Found a matching char in the map, but this relation did no match.
                                continue 'searchrelations;
                            }
                        }

                        // If fallen here, then no relations matched.

                    },
                };
            } 

            // Either no relations or char was in the map
            // Attempt to tokenize the current word as a string

            let token_start = cursor.clone();

            // Move to either next whitespace or next single char token
            skip_word::<Ctx>(src, &self.char_map, &mut cursor);

            // Take the token as a basic string or char token
            let raw_token = (&src[token_start..cursor]).to_vec();  
            if raw_token.len() > 1 {
                result_tokens.push(Token::Str(Rc::new(String::from_utf8(raw_token).expect("UTF-8 Parsing error!"))));
            } else {
                result_tokens.push(Token::Char(raw_token[0] as char));    
            }
                        
            // If left off on a whitespace, skip over it.
            skip_whitespace(src, &mut cursor);
        }

        Stack {
            stack: result_tokens
        }

    }

    pub fn add_def(&mut self, token: &str, behaviour: TokenBehaviour<Ctx>) {
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

    pub fn add_parser<P: FunctionTokenParse<ParserContext = Ctx>>(&mut self, token: &str) {
        self.add_def(token, Some(P::parse))
    }

}