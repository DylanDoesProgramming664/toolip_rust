#![allow(dead_code)]
use crate::token::{self, TokenType};
use quit;
use std::{env, process::exit};
use token::Token;

pub struct Lexer {
    input: Vec<char>,
    pos: usize,
    next_pos: usize,
    line_pos: usize,
    line_num: usize,
    char: char,
    prev_char: char,
    prev_token: Token,
}

impl Lexer {
    pub fn new(input: Vec<char>) -> Lexer {
        let mut lexer = Lexer {
            input,
            pos: 0,
            next_pos: 0,
            line_pos: 0,
            line_num: 1,
            char: '\x00',
            prev_char: '\x00',
            prev_token: Token::new(TokenType::None, "<None>".to_owned()),
        };

        lexer.next_char();

        lexer
    }

    fn peek_char(&mut self) -> char {
        if self.next_pos >= self.input.len() {
            return '\x00';
        } else {
            return self.input[self.next_pos];
        }
    }

    fn next_char(&mut self) {
        self.prev_char = self.char;
        self.char = self.peek_char();
        self.pos = self.next_pos;
        self.next_pos += 1;
        self.line_pos += 1;
    }

    pub fn next_token(&mut self) -> Token {
        let token: Token;

        if self.prev_token.Type == TokenType::NewLine {
            self.line_num += 1;
            self.line_pos = 1;
        }

        self.skip_whitespace();
        self.skip_comments();

        token = match self.char {
            '=' => {
                if self.peek_char() == '=' {
                    self.next_char();
                    Token::new(TokenType::Equals, "==".to_owned())
                } else {
                    Token::new(TokenType::Assign, "=".to_owned())
                }
            }
            '+' => {
                if self.peek_char() == '+' {
                    self.next_char();
                    Token::new(TokenType::Increment, "++".to_owned())
                } else if self.peek_char() == '=' {
                    self.next_char();
                    Token::new(TokenType::PlusEQ, "+=".to_owned())
                } else {
                    Token::new(TokenType::Plus, "+".to_owned())
                }
            }
            '-' => {
                if self.peek_char() == '-' {
                    self.next_char();
                    Token::new(TokenType::Decrement, "--".to_owned())
                } else if self.peek_char() == '=' {
                    self.next_char();
                    Token::new(TokenType::MinusEQ, "-=".to_owned())
                } else {
                    Token::new(TokenType::Minus, "-".to_owned())
                }
            }
            '*' => {
                if self.peek_char() == '*' {
                    self.next_char();
                    if self.peek_char() == '=' {
                        self.next_char();
                        Token::new(TokenType::ExpoEQ, "**=".to_owned())
                    } else {
                        Token::new(TokenType::Expo, "**".to_owned())
                    }
                } else if self.peek_char() == '=' {
                    self.next_char();
                    Token::new(TokenType::MultEQ, "*=".to_owned())
                } else {
                    Token::new(TokenType::Mult, "*".to_owned())
                }
            }
            '/' => {
                if self.peek_char() == '/' {
                    self.next_char();
                    if self.peek_char() == '=' {
                        self.next_char();
                        Token::new(TokenType::FDivEQ, "//=".to_owned())
                    } else {
                        Token::new(TokenType::FDiv, "//".to_owned())
                    }
                } else if self.peek_char() == '=' {
                    self.next_char();
                    Token::new(TokenType::DivEQ, "/=".to_owned())
                } else {
                    Token::new(TokenType::Div, "/".to_owned())
                }
            }
            '%' => {
                if self.peek_char() == '=' {
                    self.next_char();
                    Token::new(TokenType::ModEQ, "%=".to_owned())
                } else {
                    Token::new(TokenType::Mod, "%".to_owned())
                }
            }
            '!' => {
                if self.peek_char() == '=' {
                    self.next_char();
                    Token::new(TokenType::BoolNotEQ, "!=".to_owned())
                } else {
                    Token::new(TokenType::BoolNot, "!".to_owned())
                }
            }
            '~' => {
                self.next_char();
                Token::new(TokenType::BitNot, "~".to_owned())
            }
            '&' => {
                if self.peek_char() == '=' {
                    self.next_char();
                    Token::new(TokenType::BitAndEQ, "&=".to_owned())
                } else {
                    Token::new(TokenType::BitAnd, "&".to_owned())
                }
            }
            '|' => {
                if self.peek_char() == '=' {
                    self.next_char();
                    Token::new(TokenType::BitOrEQ, "|=".to_owned())
                } else {
                    Token::new(TokenType::BitOr, "|".to_owned())
                }
            }
            '^' => {
                if self.peek_char() == '=' {
                    self.next_char();
                    Token::new(TokenType::BitXorEQ, "^=".to_owned())
                } else {
                    Token::new(TokenType::BitXor, "^".to_owned())
                }
            }
            '?' => {
                if self.peek_char() == '?' {
                    self.next_char();
                    if self.peek_char() == '=' {
                        self.next_char();
                        Token::new(TokenType::CoalesceEQ, "??=".to_owned())
                    } else {
                        Token::new(TokenType::Coalesce, "??".to_owned())
                    }
                } else {
                    Token::new(TokenType::Ternary, "?".to_owned())
                }
            }
            '.' => {
                if self.peek_char() == '.' {
                    self.next_char();
                    if self.peek_char() == '.' {
                        self.next_char();
                        Token::new(TokenType::Etc, "...".to_owned())
                    } else if self.peek_char() == '=' {
                        self.next_char();
                        Token::new(TokenType::ConcatEQ, "..=".to_owned())
                    } else {
                        Token::new(TokenType::Concat, "..".to_owned())
                    }
                } else {
                    Token::new(TokenType::Dot, ".".to_owned())
                }
            }
            '<' => {
                if self.peek_char() == '<' {
                    self.next_char();
                    if self.peek_char() == '=' {
                        Token::new(TokenType::LShiftEQ, "<<=".to_owned())
                    } else {
                        Token::new(TokenType::LShift, "<<".to_owned())
                    }
                } else if self.peek_char() == '=' {
                    self.next_char();
                    Token::new(TokenType::LessThanEQ, "<=".to_owned())
                } else {
                    Token::new(TokenType::LessThan, "<".to_owned())
                }
            }
            '>' => {
                if self.peek_char() == '>' {
                    self.next_char();
                    if self.peek_char() == '=' {
                        Token::new(TokenType::RShiftEQ, ">>=".to_owned())
                    } else {
                        Token::new(TokenType::RShift, ">>".to_owned())
                    }
                } else if self.peek_char() == '=' {
                    self.next_char();
                    Token::new(TokenType::GreaterThanEQ, ">=".to_owned())
                } else {
                    Token::new(TokenType::GreaterThan, ">".to_owned())
                }
            }
            '\'' => self.next_char_string(),
            '"' => self.read_single_line_string(),
            '`' => self.read_multi_line_string(),
            ';' => Token::new(
                TokenType::Semicolon,
                self.char.clone().to_string().to_owned(),
            ),
            '(' => Token::new(TokenType::LParen, self.char.clone().to_string().to_owned()),
            ')' => Token::new(TokenType::RParen, self.char.clone().to_string().to_owned()),
            '{' => Token::new(TokenType::LBrace, self.char.clone().to_string().to_owned()),
            '}' => Token::new(TokenType::RBrace, self.char.clone().to_string().to_owned()),
            '[' => Token::new(
                TokenType::LBracket,
                self.char.clone().to_string().to_owned(),
            ),
            ']' => Token::new(
                TokenType::RBracket,
                self.char.clone().to_string().to_owned(),
            ),
            '#' => Token::new(TokenType::Hash, self.char.clone().to_string().to_owned()),
            ',' => Token::new(TokenType::Comma, self.char.clone().to_string().to_owned()),
            ':' => Token::new(TokenType::Colon, self.char.clone().to_string().to_owned()),
            '@' => Token::new(TokenType::AtSign, self.char.clone().to_string().to_owned()),
            '$' => Token::new(TokenType::Dollar, self.char.clone().to_string().to_owned()),
            '\n' => Token::new(TokenType::NewLine, self.char.clone().to_string().to_owned()),
            '\x00' => Token::new(TokenType::EOF, "".to_owned()),
            x => {
                if self.is_digit(x) {
                    self.read_number()
                } else {
                    Token::new(TokenType::Illegal, x.to_string().to_owned())
                }
            }
        };

        self.prev_token = token.clone();

        return token;
    }

    fn skip_whitespace(&mut self) {
        loop {
            match self.char {
                '\t' | '\r' | ' ' => self.next_char(),
                _ => break,
            }
        }
    }

    fn skip_comments(&mut self) {
        loop {
            if self.char == '#' {
                match self.peek_char() {
                    '#' => self.eat_line_comment(),
                    '[' => self.eat_block_comment(),
                    _ => break,
                }
            }
        }
    }

    fn eat_line_comment(&mut self) {
        loop {
            self.next_char();
            match self.char {
                '\n' => break,
                '\x00' => break,
                _ => (),
            }
        }
    }

    fn eat_block_comment(&mut self) {
        loop {
            self.next_char();
            match self.char {
                '\x00' => {
                    println!(
                        "Toolip:{}:{}: End of file reached before end of block comment.",
                        self.line_num, self.line_pos
                    )
                }
                ']' => {
                    if self.peek_char() == '#' {
                        break;
                    }
                }
                _ => (),
            }
        }
    }

    fn next_char_string(&mut self) -> Token {
        let pos = self.pos;
        let mut char_count = 0;
        self.next_char();
        loop {
            match self.char {
                '\'' => {
                    if char_count > 1 {
                        println!(
                            "Toolip:{}:{}: Too many chars for a char string.",
                            self.line_num, self.line_pos
                        );
                        if env::args().count() > 1 {
                            exit(1);
                        }
                        quit::with_code(1);
                    }
                    return Token::new(
                        TokenType::CharVal(self.input[pos] as char),
                        (self.input[pos] as char).to_string().to_owned(),
                    );
                }
                '\x00' => {
                    println!(
                        "Toolip:{}:{}: End of file reached before char string was closed.",
                        self.line_num, self.line_pos
                    );
                    if env::args().count() > 1 {
                        exit(1);
                    }
                    quit::with_code(1);
                }
                '\n' => {
                    println!(
                        "Toolip:{}:{}: New line reached before char string was closed.",
                        self.line_num, self.line_pos
                    );
                    if env::args().count() > 1 {
                        exit(1);
                    }
                    quit::with_code(1);
                }
                '\\' => {
                    self.read_escape_sequence();
                    char_count += 1;
                }
                _ => {
                    self.next_char();
                    char_count += 1;
                }
            }
        }
    }

    fn read_single_line_string(&mut self) -> Token {
        let pos = self.pos;
        self.next_char();
        loop {
            match self.char {
                '"' => {
                    return Token::new(
                        TokenType::StringVal(String::from_iter(&self.input[pos..self.pos])),
                        String::from_iter(&self.input[pos..self.pos]),
                    );
                }
                '\x00' => {
                    println!(
                        "Toolip:{}:{}: End of file reached before char string was closed.",
                        self.line_num, self.line_pos
                    );
                    if env::args().count() > 1 {
                        exit(1);
                    }
                    quit::with_code(1);
                }
                '\n' => {
                    println!(
                        "Toolip:{}:{}: New line reached before char string was closed.",
                        self.line_num, self.line_pos
                    );
                    if env::args().count() > 1 {
                        exit(1);
                    }
                    quit::with_code(1);
                }
                '\\' => {
                    self.read_escape_sequence();
                }
                _ => {
                    self.next_char();
                }
            }
        }
    }

    fn read_multi_line_string(&mut self) -> Token {
        let pos = self.pos;
        self.next_char();
        loop {
            match self.char {
                '`' => {
                    return Token::new(
                        TokenType::StringVal(String::from_iter(&self.input[pos..self.pos])),
                        String::from_iter(&self.input[pos..self.pos]),
                    );
                }
                '\x00' => {
                    println!(
                        "Toolip:{}:{}: End of file reached before char string was closed.",
                        self.line_num, self.line_pos
                    );
                    if env::args().count() > 1 {
                        exit(1);
                    }
                    quit::with_code(1);
                }
                _ => {
                    self.next_char();
                }
            }
        }
    }

    fn read_escape_sequence(&mut self) {
        match self.peek_char() {
            'n' | '\\' | '\'' | 't' | '"' => {
                self.next_char();
            }
            _ => {
                println!(
                    "Toolip:{}:{}: Invalid escape sequence.",
                    self.line_num, self.line_pos
                );
                if env::args().count() > 1 {
                    exit(1);
                }
                quit::with_code(1);
            }
        }
    }

    fn is_digit(&mut self, char: char) -> bool {
        '0' <= char && char <= '9'
    }

    fn read_number(&mut self) -> Token {
        let pos = self.pos;
        self.next_char();
        let mut is_float = false;

        loop {
            match self.char {
                '0'..='9' | '.' => {
                    if self.char == '.' && !is_float {
                        is_float = true;
                    } else if self.char == '.' {
                        break;
                    }
                    self.next_char();
                }
                _ => break,
            }
        }

        if is_float {
            return self.read_float(pos, self.pos);
        } else {
            return self.read_integer(pos, self.pos);
        }
    }

    #[allow(unused_variables)]
    fn read_float(&mut self, pos1: usize, pos2: usize) -> Token {
        todo!()
    }

    #[allow(unused_variables)]
    fn read_integer(&mut self, pos1: usize, pos2: usize) -> Token {
        todo!()
    }

    fn read_identifier(&mut self) -> Token {
        todo!()
    }
}
