use crate::token::{self, SymbolKind, Token, TokenType, KEYWORDS};
use std::{env, process::exit};

#[derive(Debug, Clone, PartialEq)]
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
            prev_token: Token::new(TokenType::Empty, "<Empty>".to_owned()),
        };

        lexer.next_char();

        return lexer;
    }

    fn peek_char(&mut self) -> char {
        if self.next_pos >= self.input.len() {
            return '\x00';
        } else {
            return self.input[self.next_pos];
        }
    }

    fn peek_nth_char(&mut self, n: usize) -> char {
        if self.pos + n >= self.input.len() {
            return '\x00';
        } else {
            return self.input[self.pos + n];
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

        let (tok_type, tok_val) = self.match_token(self.char);

        let token = Token::new(tok_type, tok_val);

        self.prev_token = token.clone();
        self.next_char();

        return token;
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

    fn match_token(&mut self, char: char) -> (TokenType, String) {
        return match char {
            '=' => match self.peek_char() {
                '=' => {
                    self.next_char();
                    (TokenType::Symbol(SymbolKind::Equals), "==".to_owned())
                }
                '>' => {
                    self.next_char();
                    (TokenType::Symbol(SymbolKind::FatArrow), "=>".to_owned())
                }
                _ => (TokenType::Symbol(SymbolKind::Assign), "=".to_owned()),
            },
            '+' => match self.peek_char() {
                '+' => {
                    self.next_char();
                    (TokenType::Symbol(SymbolKind::Increment), "++".to_owned())
                }
                '=' => {
                    self.next_char();
                    (TokenType::Symbol(SymbolKind::PlusAssign), "+=".to_owned())
                }
                _ => (TokenType::Symbol(SymbolKind::Plus), "+".to_owned()),
            },
            '-' => match self.peek_char() {
                '-' => {
                    self.next_char();
                    (TokenType::Symbol(SymbolKind::Decrement), "--".to_owned())
                }
                '=' => {
                    self.next_char();
                    (TokenType::Symbol(SymbolKind::MinusAssign), "-=".to_owned())
                }
                _ => (TokenType::Symbol(SymbolKind::Minus), "-".to_owned()),
            },
            '*' => match self.peek_char() {
                '*' => {
                    self.next_char();
                    match self.peek_char() {
                        '=' => {
                            self.next_char();
                            (
                                TokenType::Symbol(SymbolKind::ExponentAssign),
                                "**=".to_owned(),
                            )
                        }
                        _ => (TokenType::Symbol(SymbolKind::Exponent), "**".to_owned()),
                    }
                }
                '=' => {
                    self.next_char();
                    (
                        TokenType::Symbol(SymbolKind::MultiplyAssign),
                        "*=".to_owned(),
                    )
                }
                _ => (TokenType::Symbol(SymbolKind::Multiply), "*".to_owned()),
            },
            '/' => match self.peek_char() {
                '/' => {
                    self.next_char();
                    match self.peek_char() {
                        '=' => {
                            self.next_char();
                            (
                                TokenType::Symbol(SymbolKind::FloorDivideAssign),
                                "//=".to_owned(),
                            )
                        }
                        _ => (TokenType::Symbol(SymbolKind::FloorDivide), "//".to_owned()),
                    }
                }
                '=' => {
                    self.next_char();
                    (TokenType::Symbol(SymbolKind::DivideAssign), "/=".to_owned())
                }
                _ => (TokenType::Symbol(SymbolKind::Divide), "/".to_owned()),
            },
            '%' => match self.peek_char() {
                '=' => {
                    self.next_char();
                    (TokenType::Symbol(SymbolKind::ModuloAssign), "%=".to_owned())
                }
                _ => (TokenType::Symbol(SymbolKind::Modulo), "%".to_owned()),
            },
            '!' => match self.peek_char() {
                '=' => {
                    self.next_char();
                    (TokenType::Symbol(SymbolKind::BoolNotEQ), "!=".to_owned())
                }
                _ => (TokenType::Symbol(SymbolKind::BoolNot), "!".to_owned()),
            },
            '~' => (TokenType::Symbol(SymbolKind::BitNot), "~".to_owned()),
            '&' => match self.peek_char() {
                '=' => {
                    self.next_char();
                    (TokenType::Symbol(SymbolKind::BitAndAssign), "&=".to_owned())
                }
                _ => (TokenType::Symbol(SymbolKind::BitAnd), "&".to_owned()),
            },
            '|' => match self.peek_char() {
                '=' => {
                    self.next_char();
                    (TokenType::Symbol(SymbolKind::BitOrAssign), "|=".to_owned())
                }
                _ => (TokenType::Symbol(SymbolKind::BitOr), "|".to_owned()),
            },
            '^' => match self.peek_char() {
                '=' => {
                    self.next_char();
                    (TokenType::Symbol(SymbolKind::BitXorAssign), "^=".to_owned())
                }
                _ => (TokenType::Symbol(SymbolKind::BitXor), "^".to_owned()),
            },
            '?' => match self.peek_char() {
                '?' => {
                    self.next_char();
                    match self.peek_char() {
                        '=' => {
                            self.next_char();
                            (
                                TokenType::Symbol(SymbolKind::NilCoalesceAssign),
                                "??=".to_owned(),
                            )
                        }
                        _ => (TokenType::Symbol(SymbolKind::NilCoalesce), "??".to_owned()),
                    }
                }
                _ => (TokenType::Symbol(SymbolKind::Ternary), "?".to_owned()),
            },
            '.' => match self.peek_char() {
                '.' => {
                    self.next_char();
                    match self.peek_char() {
                        '.' => {
                            self.next_char();
                            match self.peek_char() {
                                '=' => (TokenType::Symbol(SymbolKind::EtcEQ), "...=".to_owned()),
                                _ => (TokenType::Symbol(SymbolKind::Etc), "...".to_owned()),
                            }
                        }
                        '=' => (
                            TokenType::Symbol(SymbolKind::ConcatAssign),
                            "..=".to_owned(),
                        ),
                        _ => (TokenType::Symbol(SymbolKind::Concat), "..".to_owned()),
                    }
                }
                _ => (TokenType::Symbol(SymbolKind::Dot), ".".to_owned()),
            },
            '<' => match self.peek_char() {
                '<' => {
                    self.next_char();
                    match self.peek_char() {
                        '=' => {
                            self.next_char();
                            (
                                TokenType::Symbol(SymbolKind::BitshiftLeftAssign),
                                "<<=".to_owned(),
                            )
                        }
                        _ => (TokenType::Symbol(SymbolKind::BitshiftLeft), "<<".to_owned()),
                    }
                }
                '=' => {
                    self.next_char();
                    (
                        TokenType::Symbol(SymbolKind::LessThanEquals),
                        "<=".to_owned(),
                    )
                }
                _ => (TokenType::Symbol(SymbolKind::LessThan), "<".to_owned()),
            },
            '>' => match self.peek_char() {
                '>' => {
                    self.next_char();
                    match self.peek_char() {
                        '=' => {
                            self.next_char();
                            (
                                TokenType::Symbol(SymbolKind::BitshiftRightAssign),
                                ">>=".to_owned(),
                            )
                        }
                        _ => (
                            TokenType::Symbol(SymbolKind::BitshiftRight),
                            ">>".to_owned(),
                        ),
                    }
                }
                '=' => {
                    self.next_char();
                    (
                        TokenType::Symbol(SymbolKind::GreaterThanEquals),
                        ">=".to_owned(),
                    )
                }
                _ => (TokenType::Symbol(SymbolKind::GreaterThan), ">".to_owned()),
            },
            '\'' => self.read_char_string(),
            '"' => self.read_single_line_string(),
            '`' => self.read_multiple_line_string(),
            ';' => (TokenType::Symbol(SymbolKind::Semicolon), ";".to_owned()),
            '(' => (TokenType::Symbol(SymbolKind::LeftParen), "(".to_owned()),
            ')' => (TokenType::Symbol(SymbolKind::RightParen), ")".to_owned()),
            '[' => (TokenType::Symbol(SymbolKind::LeftBracket), "[".to_owned()),
            ']' => (TokenType::Symbol(SymbolKind::RightBracket), "]".to_owned()),
            '{' => (TokenType::Symbol(SymbolKind::LeftBrace), "{".to_owned()),
            '}' => (TokenType::Symbol(SymbolKind::RightBrace), "}".to_owned()),
            '#' => (TokenType::Symbol(SymbolKind::Hash), "#".to_owned()),
            ',' => (TokenType::Symbol(SymbolKind::Comma), ",".to_owned()),
            ':' => (TokenType::Symbol(SymbolKind::Colon), ":".to_owned()),
            '@' => (TokenType::Symbol(SymbolKind::AtSign), "@".to_owned()),
            '$' => (TokenType::Symbol(SymbolKind::DollarSign), "$".to_owned()),
            '\n' => (TokenType::NewLine, "\n".to_owned()),
            '\x00' => (TokenType::Eof, "\x00".to_owned()),
            x => self.read_complex_token(x),
        };
    }

    fn read_char_string(&mut self) -> (TokenType, String) {
        let pos = self.pos;
        self.next_char();
        let char_count = 0;
        loop {
            match self.char {
                '\'' => {
                    if char_count > 1 {
                        if self.prev_char == '\\' {
                            self.next_char();
                            continue;
                        }
                        println!(
                            "Toolip:{}:{}: {} is not a valid char token.",
                            self.line_num,
                            self.line_pos,
                            String::from_iter(&self.input[pos..self.pos])
                        );
                        if env::args().len() > 1 {
                            exit(1);
                        }
                        return (
                            TokenType::Illegal,
                            self.input[pos..self.pos].iter().collect::<String>(),
                        );
                    }
                    break;
                }
                '\n' => {
                    println!(
                        "Toolip:{}:{}: Newline reached before char was captured.",
                        self.line_num, self.line_pos
                    );
                    if env::args().len() > 1 {
                        exit(1);
                    }
                    return (
                        TokenType::Illegal,
                        String::from_iter(&self.input[pos..self.pos]),
                    );
                }
                '\x00' => {
                    println!(
                        "Toolip:{}:{}: End of file reached before char was captured",
                        self.line_num, self.line_pos
                    );
                    if env::args().len() > 1 {
                        exit(1);
                    }
                    return (
                        TokenType::Illegal,
                        String::from_iter(&self.input[pos..self.pos]),
                    );
                }
                _ => self.next_char(),
            }
        }
        let parser_value = Self::read_string_with_escape_sequence(
            &self.input[pos + 1..self.pos].iter().collect::<Vec<_>>(),
        )
        .chars()
        .collect::<Vec<_>>()[0];
        let token_output_value = Self::read_string_with_escape_sequence(
            &self.input[pos..=self.pos].iter().collect::<Vec<_>>(),
        );
        (TokenType::CharVal(parser_value), token_output_value)
    }

    fn read_single_line_string(&mut self) -> (TokenType, String) {
        let pos = self.pos;
        self.next_char();
        loop {
            match self.char {
                '"' => {
                    if self.prev_char == '\\' {
                        self.next_char();
                        continue;
                    }
                    break;
                }
                '\n' => {
                    println!(
                        "Toolip:{}:{}: Newline found before single-line string was captured.",
                        self.line_num, self.line_pos
                    );
                    if env::args().len() > 1 {
                        exit(1);
                    }
                    return (
                        TokenType::Illegal,
                        String::from_iter(&self.input[pos..=self.pos]),
                    );
                }
                '\x00' => {
                    println!(
                        "Toolip:{}:{}: End of file reached before single-line string was captured",
                        self.line_num, self.line_pos
                    );
                    if env::args().len() > 1 {
                        exit(1);
                    }
                    return (
                        TokenType::Illegal,
                        String::from_iter(&self.input[pos..self.pos]),
                    );
                }
                _ => self.next_char(),
            }
        }
        let char_slice1 = self.input[pos + 1..self.pos].iter().collect::<Vec<_>>();
        let char_slice2 = self.input[pos..=self.pos].iter().collect::<Vec<_>>();
        let parser_value = Self::read_string_with_escape_sequence(&char_slice1);
        let token_output_value = Self::read_string_with_escape_sequence(&char_slice2);
        (TokenType::StringVal(parser_value), token_output_value)
    }

    fn read_string_with_escape_sequence(input: &Vec<&char>) -> String {
        let mut result = String::new();
        let mut index = 0;
        while index < input.len() {
            let ch = *input[index];
            if ch == '\\' {
                if index + 1 < input.len() {
                    let next_ch = input[index + 1];
                    match next_ch {
                        '\\' => {
                            result.push('\\');
                            index += 1;
                        }
                        'n' => {
                            result.push('\n');
                            index += 1;
                        }
                        't' => {
                            result.push('\t');
                            index += 1;
                        }
                        '"' => {
                            result.push('"');
                            index += 1;
                        }
                        '\'' => {
                            result.push('\'');
                            index += 1;
                        }
                        _ => {
                            result.push('\\');
                        }
                    }
                } else {
                    result.push('\\');
                }
            } else {
                result.push(ch);
            }
            index += 1;
        }
        result
    }

    fn read_multiple_line_string(&mut self) -> (TokenType, String) {
        let pos = self.pos;
        self.next_char();
        loop {
            match self.char {
                '`' => break,
                '\x00' => {
                    println!(
                        "Toolip:{}:{}: Multi-line string never terminated.",
                        self.line_num, self.line_pos
                    );
                    return (
                        TokenType::Illegal,
                        String::from_iter(&self.input[pos..self.pos]),
                    );
                }
                _ => self.next_char(),
            }
        }
        (
            TokenType::StringVal(String::from_iter(&self.input[pos + 1..self.pos])),
            String::from_iter(&self.input[pos..=self.pos]),
        )
    }

    fn read_complex_token(&mut self, char: char) -> (TokenType, String) {
        match char {
            '0'..='9' => self.read_number(),
            'a'..='z' | 'A'..='Z' => self.read_identifier(),
            x => (TokenType::Illegal, format!("{x}")),
        }
    }

    fn read_number(&mut self) -> (TokenType, String) {
        let pos = self.pos;
        let mut dot_count = 0;
        loop {
            match self.peek_char() {
                '0'..='9' => {
                    self.next_char();
                }
                '.' => {
                    if dot_count == 1
                        || self.peek_nth_char(2) == '.'
                        || !self.peek_nth_char(2).is_numeric()
                    {
                        break;
                    }
                    dot_count += 1;
                    self.next_char();
                }
                _ => break,
            }
        }

        let f64_token_value: f64 = String::from_iter(&self.input[pos..=self.pos])
            .parse::<f64>()
            .map_or_else(
                |_err| {
                    println!(
                        "Toolip:{}:{}: Could not tokenize this number!",
                        self.line_num, self.line_pos
                    );
                    exit(1);
                },
                |float: f64| float,
            );

        (
            TokenType::Float64Val(f64_token_value),
            f64_token_value.to_string(),
        )
    }

    fn read_identifier(&mut self) -> (TokenType, String) {
        let pos = self.pos;
        while let 'a'..='z' | 'A'..='Z' | '_' | '0'..='9' = self.peek_char() {
            self.next_char();
        }

        let ident = String::from_iter(&self.input[pos..=self.pos]);

        if KEYWORDS.contains(&ident.as_str()) {
            let index = KEYWORDS
                .iter()
                .position(|&x| x == ident)
                .map_or_else(|| KEYWORDS.len(), |index| index);
            return (
                TokenType::Keyword(token::match_keyword_to_index(index)),
                ident,
            );
        }
        (TokenType::Identifier(ident.to_string()), ident)
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tok = self.next_token();
        let mut tokens: Vec<Token> = vec![];
        loop {
            match tok.Type {
                TokenType::Eof => {
                    tokens.push(tok);
                    break;
                }
                TokenType::Illegal => break,
                _ => {
                    tokens.push(tok.clone());
                    tok = self.next_token();
                }
            }
        }
        tokens
    }

    pub fn print_tokens(tokens: Vec<Token>) {
        for token in tokens {
            println!("Type: {}, Value: {}", token.type_literal, token.value);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lexer_new() {
        let input = "abra kadarbra, alakazam! 123.5 ,+-="
            .chars()
            .collect::<Vec<char>>();
        let char = input[0];
        let lexer = Lexer::new(input.clone());
        let expected_lexer = Lexer {
            input,
            pos: 0,
            next_pos: 1,
            line_pos: 1,
            line_num: 1,
            char,
            prev_char: '\x00',
            prev_token: Token::new(TokenType::Empty, "<Empty>".to_owned()),
        };
        assert_eq!(lexer.input, expected_lexer.input);
        assert_eq!(lexer.pos, expected_lexer.pos);
        assert_eq!(lexer.next_pos, expected_lexer.next_pos);
        assert_eq!(lexer.line_pos, expected_lexer.line_pos);
        assert_eq!(lexer.line_num, expected_lexer.line_num);
        assert_eq!(lexer.char, expected_lexer.char);
        assert_eq!(lexer.prev_char, expected_lexer.prev_char);
        assert_eq!(lexer.prev_token, expected_lexer.prev_token);
    }
}

#[test]
fn test_tokenize() {
    let input = "abra kadarbra, alakazam! 123.5 ,+-=;\n \"sphaghetti\" c0d3 @#$%^&*()[]{}/|~"
        .chars()
        .collect::<Vec<char>>();
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize();
    let expected_tokens = vec![
        Token::new(TokenType::Identifier("abra".to_owned()), "abra".to_owned()),
        Token::new(
            TokenType::Identifier("kadarbra".to_owned()),
            "kadarbra".to_owned(),
        ),
        Token::new(TokenType::Symbol(SymbolKind::Comma), ",".to_owned()),
        Token::new(
            TokenType::Identifier("alakazam".to_owned()),
            "alakazam".to_owned(),
        ),
        Token::new(TokenType::Symbol(SymbolKind::BoolNot), "!".to_owned()),
        Token::new(TokenType::Float64Val(123.5), "123.5".to_owned()),
        Token::new(TokenType::Symbol(SymbolKind::Comma), ",".to_owned()),
        Token::new(TokenType::Symbol(SymbolKind::Plus), "+".to_owned()),
        Token::new(TokenType::Symbol(SymbolKind::MinusAssign), "-=".to_owned()),
        Token::new(TokenType::Symbol(SymbolKind::Semicolon), ";".to_owned()),
        Token::new(TokenType::NewLine, "\n".to_owned()),
        Token::new(
            TokenType::StringVal("sphaghetti".to_owned()),
            "\"sphaghetti\"".to_owned(),
        ),
        Token::new(TokenType::Identifier("c0d3".to_owned()), "c0d3".to_owned()),
        Token::new(TokenType::Symbol(SymbolKind::AtSign), "@".to_owned()),
        Token::new(TokenType::Symbol(SymbolKind::Hash), "#".to_owned()),
        Token::new(TokenType::Symbol(SymbolKind::DollarSign), "$".to_owned()),
        Token::new(TokenType::Symbol(SymbolKind::Modulo), "%".to_owned()),
        Token::new(TokenType::Symbol(SymbolKind::BitXor), "^".to_owned()),
        Token::new(TokenType::Symbol(SymbolKind::BitAnd), "&".to_owned()),
        Token::new(TokenType::Symbol(SymbolKind::Multiply), "*".to_owned()),
        Token::new(TokenType::Symbol(SymbolKind::LeftParen), "(".to_owned()),
        Token::new(TokenType::Symbol(SymbolKind::RightParen), ")".to_owned()),
        Token::new(TokenType::Symbol(SymbolKind::LeftBracket), "[".to_owned()),
        Token::new(TokenType::Symbol(SymbolKind::RightBracket), "]".to_owned()),
        Token::new(TokenType::Symbol(SymbolKind::LeftBrace), "{".to_owned()),
        Token::new(TokenType::Symbol(SymbolKind::RightBrace), "}".to_owned()),
        Token::new(TokenType::Symbol(SymbolKind::Divide), "/".to_owned()),
        Token::new(TokenType::Symbol(SymbolKind::BitOr), "|".to_owned()),
        Token::new(TokenType::Symbol(SymbolKind::BitNot), "~".to_owned()),
        Token::new(TokenType::Eof, "\x00".to_owned()),
    ];
    for i in 0..tokens.len() {
        assert_eq!(tokens[i], expected_tokens[i]);
    }
}
