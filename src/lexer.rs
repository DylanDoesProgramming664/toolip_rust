#![allow(unused_imports)]
use crate::parser;
use crate::token::{self, Type};
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
        let mut lexer = Self {
            input,
            pos: 0,
            next_pos: 0,
            line_pos: 0,
            line_num: 1,
            char: '\x00',
            prev_char: '\x00',
            prev_token: Token::new(Type::None, "<None>".to_owned()),
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
        if self.prev_token.Type == Type::NewLine {
            self.line_num += 1;
            self.line_pos = 1;
        }

        self.skip_whitespace();
        self.skip_comments();

        let token = self.match_token(self.char);

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

    fn match_token(&mut self, char: char) -> Token {
        match char {
            '=' => match self.peek_char() {
                '=' => {
                    self.next_char();
                    Token::new(Type::Equals, "==".to_owned())
                }
                '>' => {
                    self.next_char();
                    Token::new(Type::ThickArrow, "=>".to_owned())
                }
                _ => Token::new(Type::Assign, "=".to_owned()),
            },
            '+' => match self.peek_char() {
                '+' => {
                    self.next_char();
                    Token::new(Type::Increment, "++".to_owned())
                }
                '=' => {
                    self.next_char();
                    Token::new(Type::PlusEQ, "+=".to_owned())
                }
                _ => Token::new(Type::Plus, "+".to_owned()),
            },
            '-' => match self.peek_char() {
                '-' => {
                    self.next_char();
                    Token::new(Type::Decrement, "--".to_owned())
                }
                '=' => {
                    self.next_char();
                    Token::new(Type::MinusEQ, "-=".to_owned())
                }
                '>' => {
                    self.next_char();
                    Token::new(Type::ThinArrow, "->".to_owned())
                }
                _ => Token::new(Type::Minus, "-".to_owned()),
            },
            '*' => match self.peek_char() {
                '*' => {
                    self.next_char();
                    match self.peek_char() {
                        '=' => {
                            self.next_char();
                            Token::new(Type::ExpoEQ, "**=".to_owned())
                        }
                        _ => Token::new(Type::Expo, "**".to_owned()),
                    }
                }
                '=' => {
                    self.next_char();
                    Token::new(Type::MultEQ, "*=".to_owned())
                }
                _ => Token::new(Type::Mult, "*".to_owned()),
            },
            '/' => match self.peek_char() {
                '/' => {
                    self.next_char();
                    match self.peek_char() {
                        '=' => {
                            self.next_char();
                            Token::new(Type::FDivEQ, "//=".to_owned())
                        }
                        _ => Token::new(Type::FDiv, "//".to_owned()),
                    }
                }
                '=' => {
                    self.next_char();
                    Token::new(Type::DivEQ, "/=".to_owned())
                }
                _ => Token::new(Type::Div, "/".to_owned()),
            },
            '%' => match self.peek_char() {
                '=' => {
                    self.next_char();
                    Token::new(Type::ModEQ, "%=".to_owned())
                }
                _ => Token::new(Type::Mod, "%".to_owned()),
            },
            '!' => match self.peek_char() {
                '=' => {
                    self.next_char();
                    Token::new(Type::BoolNotEQ, "!=".to_owned())
                }
                _ => Token::new(Type::BoolNot, "!".to_owned()),
            },
            '~' => Token::new(Type::BitNot, "~".to_owned()),
            '&' => match self.peek_char() {
                '=' => {
                    self.next_char();
                    Token::new(Type::BitAndEQ, "&=".to_owned())
                }
                _ => Token::new(Type::BitAnd, "&".to_owned()),
            },
            '|' => match self.peek_char() {
                '=' => {
                    self.next_char();
                    Token::new(Type::BitOrEQ, "|=".to_owned())
                }
                _ => Token::new(Type::BitOr, "|".to_owned()),
            },
            '^' => match self.peek_char() {
                '=' => {
                    self.next_char();
                    Token::new(Type::BitXorEQ, "^=".to_owned())
                }
                _ => Token::new(Type::BitXor, "^".to_owned()),
            },
            '?' => match self.peek_char() {
                '?' => {
                    self.next_char();
                    match self.peek_char() {
                        '=' => {
                            self.next_char();
                            Token::new(Type::CoalesceEQ, "??=".to_owned())
                        }
                        _ => Token::new(Type::Coalesce, "??".to_owned()),
                    }
                }
                _ => Token::new(Type::Ternary, "?".to_owned()),
            },
            '.' => match self.peek_char() {
                '.' => {
                    self.next_char();
                    match self.peek_char() {
                        '.' => {
                            self.next_char();
                            Token::new(Type::Etc, "...".to_owned())
                        }
                        '=' => {
                            self.next_char();
                            Token::new(Type::ConcatEQ, "..=".to_owned())
                        }
                        _ => Token::new(Type::Concat, "..".to_owned()),
                    }
                }
                _ => Token::new(Type::Dot, ".".to_owned()),
            },
            '<' => match self.peek_char() {
                '<' => {
                    self.next_char();
                    match self.peek_char() {
                        '=' => Token::new(Type::LShiftEQ, "<<=".to_owned()),
                        _ => Token::new(Type::LShift, "<<".to_owned()),
                    }
                }
                '=' => {
                    self.next_char();
                    Token::new(Type::LessThanEQ, "<=".to_owned())
                }
                _ => Token::new(Type::LessThan, "<".to_owned()),
            },
            '>' => match self.peek_char() {
                '>' => {
                    self.next_char();
                    match self.peek_char() {
                        '=' => Token::new(Type::RShiftEQ, ">>=".to_owned()),
                        _ => Token::new(Type::RShift, ">>".to_owned()),
                    }
                }
                '=' => {
                    self.next_char();
                    Token::new(Type::GreaterThanEQ, ">=".to_owned())
                }
                _ => Token::new(Type::GreaterThan, ">".to_owned()),
            },
            '\'' => self.next_char_string(),
            '"' => self.read_single_line_string(),
            '`' => self.read_multi_line_string(),
            ';' => Token::new(Type::Semicolon, self.char.clone().to_string()),
            '(' => Token::new(Type::LParen, self.char.clone().to_string()),
            ')' => Token::new(Type::RParen, self.char.clone().to_string()),
            '{' => Token::new(Type::LBrace, self.char.clone().to_string()),
            '}' => Token::new(Type::RBrace, self.char.clone().to_string()),
            '[' => Token::new(Type::LBracket, self.char.clone().to_string()),
            ']' => Token::new(Type::RBracket, self.char.clone().to_string()),
            '#' => Token::new(Type::Hash, self.char.clone().to_string()),
            ',' => Token::new(Type::Comma, self.char.clone().to_string()),
            ':' => Token::new(Type::Colon, self.char.clone().to_string()),
            '@' => Token::new(Type::AtSign, self.char.clone().to_string()),
            '$' => Token::new(Type::Dollar, self.char.clone().to_string()),
            '\n' => Token::new(Type::NewLine, self.char.clone().to_string()),
            '\x00' => Token::new(Type::Eof, String::new()),
            x => {
                if self.is_digit(x) {
                    self.read_number()
                } else if self.is_letter(x) {
                    self.read_identifier()
                } else {
                    Token::new(Type::Illegal, x.to_string())
                }
            }
        }
    }

    fn eat_line_comment(&mut self) {
        loop {
            self.next_char();
            match self.char {
                '\n' | '\x00' => break,
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
                    );
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
                            Type::Illegal,
                            String::from_iter(&self.input[pos..self.pos]),
                        );
                    }
                    return Token::new(
                        Type::CharVal(self.input[pos + 1]),
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
                        Type::Illegal,
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
                        Type::Illegal,
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
                        Type::StringVal(String::from_iter(&self.input[pos..=self.pos])),
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
                        Type::Illegal,
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
                        Type::Illegal,
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
                        Type::StringVal(String::from_iter(&self.input[pos..self.pos])),
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
                        Type::Illegal,
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
            "global" => Type::Global,
            "const" => Type::Const,
            "static" => Type::Static,
            "Jit" => Type::Jit,
            "unsafe" => Type::Unsafe,
            "coroutine" => Type::Coroutine,
            "func" => Type::Func,
            "this" => Type::This,
            "struct" => Type::Struct,
            "self" => Type::Slf,
            "end" => Type::End,
            "if" => Type::If,
            "else" => Type::Else,
            "elseif" => Type::ElseIf,
            "then" => Type::Then,
            "for" => Type::For,
            "while" => Type::While,
            "loop" => Type::Loop,
            "break" => Type::Break,
            "match" => Type::Match,
            "to" => Type::To,
            "in" => Type::In,
            "with" => Type::With,
            "bool" => Type::BoolType,
            "uint8" => Type::UInt8Type,
            "uint16" => Type::UInt16Type,
            "uint32" => Type::UInt32Type,
            "uint64" => Type::UInt64Type,
            "uint128" => Type::UInt128Type,
            "int8" => Type::Int8Type,
            "int16" => Type::Int16Type,
            "int32" => Type::Int32Type,
            "int64" => Type::Int64Type,
            "int128" => Type::Int128Type,
            "flt32" => Type::Float32Type,
            "flt64" => Type::Float64Type,
            "char" => Type::CharType,
            "string" => Type::StringType,
            "array" => Type::Array,
            "table" => Type::Table,
            "stack" => Type::Stack,
            "queue" => Type::Queue,
            "set" => Type::Set,
            "list" => Type::List,
            "twolist" => Type::TwoList,
            "heap" => Type::Heap,
            "tree" => Type::Tree,
            "enum" => Type::Enum,
            "type" => Type::Type,
            "return" => Type::Return,
            "and" => Type::And,
            "or" => Type::Or,
            "not" => Type::Not,
            "xor" => Type::Xor,
            "true" => Type::True,
            "false" => Type::False,
            "nil" => Type::Nil,
            _ => Type::Identifier,
        };

        Token::new(token_type, str.as_str().to_string())
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tok = self.next_token();
        let mut tokens: Vec<Token> = vec![];
        loop {
            match tok.Type {
                Type::Eof => {
                    tokens.push(tok);
                    break;
                }
                Type::Illegal => break,
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
        println!("Type: {}, TokenValue: {}", &tok.type_literal, &tok.value);
        assert_eq!(tok.Type, Type::Int64Type);
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
                Type: Type::StringType,
                type_literal: "StringType".to_string(),
                value: "string".to_string(),
            },
            Token {
                Type: Type::Identifier,
                type_literal: "Identifier".to_string(),
                value: "message".to_string(),
            },
            Token {
                Type: Type::Assign,
                type_literal: "=".to_string(),
                value: "=".to_string(),
            },
            Token {
                Type: Type::StringVal("\"Welcome to the Toolip programming language\"".to_string()),
                type_literal: "StringVal".to_string(),
                value: "\"Welcome to the Toolip programming language\"".to_string(),
            },
            Token::new(Type::Eof, String::new()),
        ];
        assert_eq!(tokens, expected);
    }
}
