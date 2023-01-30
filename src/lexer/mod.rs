use crate::token;

pub struct Lexer<'a> {
    input: &'a str,
    pos: usize,
    next_pos: usize,
    line_pos: usize,
    line_num: usize,
    char: u8,
}

struct LexerStats {
    prev_token: token::Token,
    prev_char: u8,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Lexer {
            input,
            pos: 0,
            next_pos: 0,
            line_pos: 0,
            line_num: 0,
            char: 0,
        }
    }
}

impl LexerStats {
    fn new(prev_token: token::Token, prev_char: u8) -> Self {
        LexerStats {
            prev_token,
            prev_char,
        }
    }

    pub fn update_prev_token(&mut self, curr_token: token::Token) {
        self.prev_token = curr_token;
    }
}
