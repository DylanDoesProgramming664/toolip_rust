#![allow(unused_imports)]
use crate::parser;
use crate::token::{self, TokenType};
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
    pub fn new(input: Vec<char>) -> Self {
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
            '\x00'
        } else {
            self.input[self.next_pos]
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
        if self.prev_token.Type == TokenType::NewLine {
            self.line_num += 1;
            self.line_pos = 1;
        }

        self.skip_whitespace();
        self.skip_comments();

        let token = match self.char {
            '=' => match self.peek_char() {
                '=' => {
                    self.next_char();
                    Token::new(TokenType::Equals, "==".to_owned())
                }
                '>' => {
                    self.next_char();
                    Token::new(TokenType::ThickArrow, "=>".to_owned())
                }
                _ => Token::new(TokenType::Assign, "=".to_owned()),
            },
            '+' => match self.peek_char() {
                '+' => {
                    self.next_char();
                    Token::new(TokenType::Increment, "++".to_owned())
                }
                '=' => {
                    self.next_char();
                    Token::new(TokenType::PlusEQ, "+=".to_owned())
                }
                _ => Token::new(TokenType::Plus, "+".to_owned()),
            },
            '-' => match self.peek_char() {
                '-' => {
                    self.next_char();
                    Token::new(TokenType::Decrement, "--".to_owned())
                }
                '=' => {
                    self.next_char();
                    Token::new(TokenType::MinusEQ, "-=".to_owned())
                }
                '>' => {
                    self.next_char();
                    Token::new(TokenType::ThinArrow, "->".to_owned())
                }
                _ => Token::new(TokenType::Minus, "-".to_owned()),
            },
            '*' => match self.peek_char() {
                '*' => {
                    self.next_char();
                    match self.peek_char() {
                        '=' => {
                            self.next_char();
                            Token::new(TokenType::ExpoEQ, "**=".to_owned())
                        }
                        _ => Token::new(TokenType::Expo, "**".to_owned()),
                    }
                }
                '=' => {
                    self.next_char();
                    Token::new(TokenType::MultEQ, "*=".to_owned())
                }
                _ => Token::new(TokenType::Mult, "*".to_owned()),
            },
            '/' => match self.peek_char() {
                '/' => {
                    self.next_char();
                    match self.peek_char() {
                        '=' => {
                            self.next_char();
                            Token::new(TokenType::FDivEQ, "//=".to_owned())
                        }
                        _ => Token::new(TokenType::FDiv, "//".to_owned()),
                    }
                }
                '=' => {
                    self.next_char();
                    Token::new(TokenType::DivEQ, "/=".to_owned())
                }
                _ => Token::new(TokenType::Div, "/".to_owned()),
            },
            '%' => match self.peek_char() {
                '=' => {
                    self.next_char();
                    Token::new(TokenType::ModEQ, "%=".to_owned())
                }
                _ => Token::new(TokenType::Mod, "%".to_owned()),
            },
            '!' => match self.peek_char() {
                '=' => {
                    self.next_char();
                    Token::new(TokenType::BoolNotEQ, "!=".to_owned())
                }
                _ => Token::new(TokenType::BoolNot, "!".to_owned()),
            },
            '~' => Token::new(TokenType::BitNot, "~".to_owned()),
            '&' => match self.peek_char() {
                '=' => {
                    self.next_char();
                    Token::new(TokenType::BitAndEQ, "&=".to_owned())
                }
                _ => Token::new(TokenType::BitAnd, "&".to_owned()),
            },
            '|' => match self.peek_char() {
                '=' => {
                    self.next_char();
                    Token::new(TokenType::BitOrEQ, "|=".to_owned())
                }
                _ => Token::new(TokenType::BitOr, "|".to_owned()),
            },
            '^' => match self.peek_char() {
                '=' => {
                    self.next_char();
                    Token::new(TokenType::BitXorEQ, "^=".to_owned())
                }
                _ => Token::new(TokenType::BitXor, "^".to_owned()),
            },
            '?' => match self.peek_char() {
                '?' => {
                    self.next_char();
                    match self.peek_char() {
                        '=' => {
                            self.next_char();
                            Token::new(TokenType::CoalesceEQ, "??=".to_owned())
                        }
                        _ => Token::new(TokenType::Coalesce, "??".to_owned()),
                    }
                }
                _ => Token::new(TokenType::Ternary, "?".to_owned()),
            },
            '.' => match self.peek_char() {
                '.' => {
                    self.next_char();
                    match self.peek_char() {
                        '.' => {
                            self.next_char();
                            Token::new(TokenType::Etc, "...".to_owned())
                        }
                        '=' => {
                            self.next_char();
                            Token::new(TokenType::ConcatEQ, "..=".to_owned())
                        }
                        _ => Token::new(TokenType::Concat, "..".to_owned()),
                    }
                }
                _ => Token::new(TokenType::Dot, ".".to_owned()),
            },
            '<' => match self.peek_char() {
                '<' => {
                    self.next_char();
                    match self.peek_char() {
                        '=' => Token::new(TokenType::LShiftEQ, "<<=".to_owned()),
                        _ => Token::new(TokenType::LShift, "<<".to_owned()),
                    }
                }
                '=' => {
                    self.next_char();
                    Token::new(TokenType::LessThanEQ, "<=".to_owned())
                }
                _ => Token::new(TokenType::LessThan, "<".to_owned()),
            },
            '>' => match self.peek_char() {
                '>' => {
                    self.next_char();
                    match self.peek_char() {
                        '=' => Token::new(TokenType::RShiftEQ, ">>=".to_owned()),
                        _ => Token::new(TokenType::RShift, ">>".to_owned()),
                    }
                }
                '=' => {
                    self.next_char();
                    Token::new(TokenType::GreaterThanEQ, ">=".to_owned())
                }
                _ => Token::new(TokenType::GreaterThan, ">".to_owned()),
            },
            '\'' => self.next_char_string(),
            '"' => self.read_single_line_string(),
            '`' => self.read_multi_line_string(),
            ';' => Token::new(TokenType::Semicolon, self.char.clone().to_string()),
            '(' => Token::new(TokenType::LParen, self.char.clone().to_string()),
            ')' => Token::new(TokenType::RParen, self.char.clone().to_string()),
            '{' => Token::new(TokenType::LBrace, self.char.clone().to_string()),
            '}' => Token::new(TokenType::RBrace, self.char.clone().to_string()),
            '[' => Token::new(TokenType::LBracket, self.char.clone().to_string()),
            ']' => Token::new(TokenType::RBracket, self.char.clone().to_string()),
            '#' => Token::new(TokenType::Hash, self.char.clone().to_string()),
            ',' => Token::new(TokenType::Comma, self.char.clone().to_string()),
            ':' => Token::new(TokenType::Colon, self.char.clone().to_string()),
            '@' => Token::new(TokenType::AtSign, self.char.clone().to_string()),
            '$' => Token::new(TokenType::Dollar, self.char.clone().to_string()),
            '\n' => Token::new(TokenType::NewLine, self.char.clone().to_string()),
            '\x00' => Token::new(TokenType::Eof, "".to_owned()),
            x => {
                if self.is_digit(x) {
                    self.read_number()
                } else if self.is_letter(x) {
                    self.read_identifier()
                } else {
                    Token::new(TokenType::Illegal, x.to_string())
                }
            }
        };

        self.prev_token = token.clone();
        self.next_char();

        token
    }

    fn skip_whitespace(&mut self) {
        while let '\t' | '\r' | ' ' = self.char {
            self.next_char();
        }
    }

    fn skip_comments(&mut self) {
        if self.char == '#' {
            loop {
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
                        return Token::new(
                            TokenType::Illegal,
                            String::from_iter(&self.input[pos..self.pos]),
                        );
                    }
                    return Token::new(
                        TokenType::CharVal(self.input[pos + 1]),
                        format!("'{}'", self.input[pos + 1]),
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
                    return Token::new(
                        TokenType::Illegal,
                        String::from_iter(&self.input[pos..self.pos]),
                    );
                }
                '\n' => {
                    println!(
                        "Toolip:{}:{}: New line reached before char string was closed.",
                        self.line_num, self.line_pos
                    );
                    if env::args().count() > 1 {
                        exit(1);
                    }
                    return Token::new(
                        TokenType::Illegal,
                        String::from_iter(&self.input[pos..self.pos]),
                    );
                }
                '\\' => {
                    self.read_escape_sequence();
                    self.next_char();
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
                        TokenType::StringVal(String::from_iter(&self.input[pos..=self.pos])),
                        String::from_iter(&self.input[pos..=self.pos]),
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
                    return Token::new(
                        TokenType::Illegal,
                        String::from_iter(&self.input[pos..self.pos]),
                    );
                }
                '\n' => {
                    println!(
                        "Toolip:{}:{}: New line reached before char string was closed.",
                        self.line_num, self.line_pos
                    );
                    if env::args().count() > 1 {
                        exit(1);
                    }
                    return Token::new(
                        TokenType::Illegal,
                        String::from_iter(&self.input[pos..self.pos]),
                    );
                }
                '\\' => {
                    self.read_escape_sequence();
                    self.next_char();
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
                    return Token::new(
                        TokenType::Illegal,
                        String::from_iter(&self.input[pos..self.pos]),
                    );
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
            }
        }
    }

    fn is_digit(&mut self, char: char) -> bool {
        ('0'..='9').contains(&char)
    }

    fn read_number(&mut self) -> Token {
        let pos = self.pos;
        self.next_char();
        let mut is_float = false;

        while let '0'..='9' | '.' = self.char {
            if self.char == '.' && !is_float {
                is_float = true;
            } else if self.char == '.' {
                break;
            }
            self.next_char();
        }

        if is_float {
            self.read_float(pos, self.pos)
        } else {
            self.read_integer(pos, self.pos)
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

    fn is_letter(&mut self, char: char) -> bool {
        char.is_ascii_alphabetic()
    }

    fn read_identifier(&mut self) -> Token {
        let pos = self.pos;
        let mut next_char = self.peek_char();
        if self.is_non_starting_identifier(next_char) {
            self.next_char();
            loop {
                next_char = self.peek_char();
                if self.is_non_starting_identifier(self.char)
                    && self.is_non_starting_identifier(next_char)
                {
                    self.next_char();
                } else {
                    break;
                }
            }
        }
        self.match_identifier(pos, self.pos)
    }

    fn is_non_starting_identifier(&mut self, char: char) -> bool {
        self.is_letter(char) || char == '_' || self.is_digit(char)
    }

    fn match_identifier(&mut self, pos1: usize, pos2: usize) -> Token {
        let str = String::from_iter(&self.input[pos1..=pos2]);

        let token_type = match str.as_str() {
            "global" => TokenType::Global,
            "const" => TokenType::Const,
            "static" => TokenType::Static,
            "Jit" => TokenType::Jit,
            "unsafe" => TokenType::Unsafe,
            "coroutine" => TokenType::Coroutine,
            "func" => TokenType::Func,
            "this" => TokenType::This,
            "struct" => TokenType::Struct,
            "self" => TokenType::Slf,
            "end" => TokenType::End,
            "if" => TokenType::If,
            "else" => TokenType::Else,
            "elseif" => TokenType::ElseIf,
            "then" => TokenType::Then,
            "for" => TokenType::For,
            "while" => TokenType::While,
            "loop" => TokenType::Loop,
            "break" => TokenType::Break,
            "match" => TokenType::Match,
            "to" => TokenType::To,
            "in" => TokenType::In,
            "with" => TokenType::With,
            "bool" => TokenType::BoolType,
            "uint8" => TokenType::UInt8Type,
            "uint16" => TokenType::UInt16Type,
            "uint32" => TokenType::UInt32Type,
            "uint64" => TokenType::UInt64Type,
            "uint128" => TokenType::UInt128Type,
            "int8" => TokenType::Int8Type,
            "int16" => TokenType::Int16Type,
            "int32" => TokenType::Int32Type,
            "int64" => TokenType::Int64Type,
            "int128" => TokenType::Int128Type,
            "flt32" => TokenType::Float32Type,
            "flt64" => TokenType::Float64Type,
            "char" => TokenType::CharType,
            "string" => TokenType::StringType,
            "array" => TokenType::Array,
            "table" => TokenType::Table,
            "stack" => TokenType::Stack,
            "queue" => TokenType::Queue,
            "set" => TokenType::Set,
            "list" => TokenType::List,
            "twolist" => TokenType::TwoList,
            "heap" => TokenType::Heap,
            "tree" => TokenType::Tree,
            "enum" => TokenType::Enum,
            "type" => TokenType::Type,
            "return" => TokenType::Return,
            "and" => TokenType::And,
            "or" => TokenType::Or,
            "not" => TokenType::Not,
            "xor" => TokenType::Xor,
            "true" => TokenType::True,
            "false" => TokenType::False,
            "nil" => TokenType::Nil,
            _ => TokenType::Identifier,
        };

        Token::new(token_type, str.as_str().to_string())
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tok = self.next_token();
        let mut tokens: Vec<Token> = vec![];
        loop {
            match tok.Type {
                TokenType::Eof | TokenType::Illegal => break,
                _ => {
                    tokens.push(tok.clone());
                    tok = self.next_token();
                }
            }
        }
        tokens
    }

    pub fn print_tokens(&mut self) {
        let tokens = self.tokenize();
        for tok in tokens {
            println!("(Type:{}, Value:{})", tok.type_literal, tok.value);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lexer_new() {
        let mut lexer = Lexer::new("int64".chars().collect::<Vec<char>>());
        let tok = lexer.next_token();
        println!(
            "TokenType: {}, TokenValue: {}",
            &tok.type_literal, &tok.value
        );
        assert_eq!(tok.Type, TokenType::Int64Type);
    }

    #[test]
    fn test_lexer_line() {
        let input = "string message = \"Welcome to the Toolip programming language\""
            .chars()
            .collect::<Vec<char>>();
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize();
        let expected = vec![
            Token {
                Type: TokenType::StringType,
                type_literal: "StringType".to_string(),
                value: "string".to_string(),
            },
            Token {
                Type: TokenType::Identifier,
                type_literal: "Identifier".to_string(),
                value: "message".to_string(),
            },
            Token {
                Type: TokenType::Assign,
                type_literal: "=".to_string(),
                value: "=".to_string(),
            },
            Token {
                Type: TokenType::StringVal(
                    "\"Welcome to the Toolip programming language\"".to_string(),
                ),
                type_literal: "StringVal".to_string(),
                value: "\"Welcome to the Toolip programming language\"".to_string(),
            },
        ];
        assert_eq!(tokens, expected);
    }
}
