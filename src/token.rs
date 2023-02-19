#![allow(dead_code, non_snake_case)]
use crate::lexer::Lexer;
#[derive(Clone, PartialEq, Debug)]
pub struct Token {
    pub Type: TokenType,
    pub type_literal: String,
    pub value: String,
}

impl Token {
    pub fn new(tok_type: TokenType, value: String) -> Token {
        let tok_lit = get_type_literal(&tok_type);
        Token {
            Type: tok_type,
            type_literal: tok_lit,
            value,
        }
    }

    pub fn new_type(&mut self, tok_type: TokenType) {
        self.type_literal = get_type_literal(&tok_type);
        self.Type = tok_type;
    }

    pub fn reevaluate_token(&mut self, tok_value: String) {
        let input = tok_value.as_str().chars().collect::<Vec<char>>();
        let mut sub_lexer = Lexer::new(input);
        let tok = sub_lexer.next_token();
        self.type_literal = get_type_literal(&tok.Type);
        self.Type = tok.Type;
        self.value = tok.value;
    }
}

#[derive(PartialEq, Clone, Debug)]
pub enum TokenType {
    None,
    Illegal,
    NewLine,
    Eof,

    /* Identifiers & Literals */
    Identifier,
    BoolVal(bool),
    UInt8Val(u8),
    UInt16Val(u16),
    UInt32Val(u32),
    UInt64Val(u64),
    UInt128Val(u128),
    Int8Val(i8),
    Int16Val(i16),
    Int32Val(i32),
    Int64Val(i64),
    Int128Val(i128),
    Float32Val(f32),
    Float64Val(f64),
    CharVal(char),
    StringVal(String),

    /* Operators */
    Equals,
    Assign,
    Increment,
    Decrement,
    LShiftEQ,
    RShiftEQ,
    PlusEQ,
    MinusEQ,
    ExpoEQ,
    MultEQ,
    FDivEQ,
    DivEQ,
    ModEQ,
    ConcatEQ,
    CoalesceEQ,
    LessThanEQ,
    GreaterThanEQ,
    BoolNotEQ,
    BitAndEQ,
    BitOrEQ,
    BitXorEQ,
    LShift,
    RShift,
    Plus,
    Minus,
    Expo,
    Mult,
    FDiv,
    Div,
    Mod,
    Concat,
    Coalesce,
    Ternary,
    LessThan,
    GreaterThan,
    BoolNot,
    BitNot,
    BitAnd,
    BitOr,
    BitXor,
    Hash,

    /* Delimiters */
    Semicolon,
    Comma,
    Dot,
    Colon,
    ThinArrow,
    ThickArrow,
    AtSign,
    Dollar,
    LParen,
    RParen,
    LBrace,
    RBrace,
    LBracket,
    RBracket,
    Etc,

    /* Keywords */
    Global,
    Const,
    Static,
    Jit,
    Unsafe,
    Coroutine,
    Func,
    This,
    End,
    Struct,
    Slf, // Can't write Self, so removed a letter out of spite :(
    If,
    Then,
    Else,
    ElseIf,
    For,
    Do,
    While,
    Loop,
    Break,
    To,
    Match,
    In,
    With,
    BoolType,
    UInt8Type,
    UInt16Type,
    UInt32Type,
    UInt64Type,
    UInt128Type,
    Int8Type,
    Int16Type,
    Int32Type,
    Int64Type,
    Int128Type,
    Float32Type,
    Float64Type,
    CharType,
    StringType,
    Array,
    Table,
    Stack,
    Queue,
    Set, // Stack-Queue Hybrid
    List,
    TwoList,
    Heap,
    Graph,
    Tree,
    Enum,
    Type,
    Return,
    And,
    Or,
    Not,
    Xor,
    True,
    False,
    Nil,
}

pub fn get_type_literal(tok_type: &TokenType) -> String {
    match tok_type {
        TokenType::None => "<None>",
        TokenType::Illegal => "<Illegal>",
        TokenType::NewLine => "<NewLine>",
        TokenType::Eof => "<Eof>",
        TokenType::Identifier => "Identifier",
        TokenType::BoolVal(_) => "BoolVal",
        TokenType::UInt8Val(_) => "UInt8Val",
        TokenType::UInt16Val(_) => "UInt16Val",
        TokenType::UInt32Val(_) => "UInt32Val",
        TokenType::UInt64Val(_) => "UInt64Val",
        TokenType::UInt128Val(_) => "UInt128Val",
        TokenType::Int8Val(_) => "Int8Val",
        TokenType::Int16Val(_) => "Int16Val",
        TokenType::Int32Val(_) => "Int32Val",
        TokenType::Int64Val(_) => "Int64Val",
        TokenType::Int128Val(_) => "Int128Val",
        TokenType::Float32Val(_) => "Float32Val",
        TokenType::Float64Val(_) => "Float64Val",
        TokenType::CharVal(_) => "CharVal",
        TokenType::StringVal(_) => "StringVal",
        TokenType::Equals => "==",
        TokenType::Assign => "=",
        TokenType::Increment => "++",
        TokenType::Decrement => "--",
        TokenType::LShiftEQ => "<<=",
        TokenType::RShiftEQ => ">>=",
        TokenType::PlusEQ => "+=",
        TokenType::MinusEQ => "-=",
        TokenType::ExpoEQ => "**=",
        TokenType::MultEQ => "*=",
        TokenType::FDivEQ => "//=",
        TokenType::DivEQ => "/=",
        TokenType::ModEQ => "%=",
        TokenType::ConcatEQ => "..=",
        TokenType::CoalesceEQ => "??=",
        TokenType::LessThanEQ => "<=",
        TokenType::GreaterThanEQ => ">=",
        TokenType::BoolNotEQ => "!=",
        TokenType::BitAndEQ => "&=",
        TokenType::BitOrEQ => "|=",
        TokenType::BitXorEQ => "^=",
        TokenType::LShift => "<<",
        TokenType::RShift => ">>",
        TokenType::Plus => "+",
        TokenType::Minus => "-",
        TokenType::Expo => "**",
        TokenType::Mult => "*",
        TokenType::FDiv => "//",
        TokenType::Div => "/",
        TokenType::Mod => "%",
        TokenType::Concat => "..",
        TokenType::Coalesce => "??",
        TokenType::Ternary => "?",
        TokenType::LessThan => "<",
        TokenType::GreaterThan => ">",
        TokenType::BoolNot => "!",
        TokenType::BitNot => "~",
        TokenType::BitAnd => "&",
        TokenType::BitOr => "|",
        TokenType::BitXor => "^",
        TokenType::Hash => "#",
        TokenType::Semicolon => ";",
        TokenType::Comma => ",",
        TokenType::Dot => ".",
        TokenType::Colon => ":",
        TokenType::ThinArrow => "->",
        TokenType::ThickArrow => "=>",
        TokenType::AtSign => "@",
        TokenType::Dollar => "$",
        TokenType::LParen => "(",
        TokenType::RParen => ")",
        TokenType::LBrace => "{",
        TokenType::RBrace => "}",
        TokenType::LBracket => "[",
        TokenType::RBracket => "]",
        TokenType::Global => "Global",
        TokenType::Const => "Const",
        TokenType::Static => "Static",
        TokenType::Jit => "Jit",
        TokenType::Unsafe => "Unsafe",
        TokenType::Coroutine => "Coroutine",
        TokenType::Func => "Func",
        TokenType::This => "This",
        TokenType::End => "End",
        TokenType::Slf => "Self",
        TokenType::If => "If",
        TokenType::Then => "Then",
        TokenType::Else => "Else",
        TokenType::ElseIf => "ElseIf",
        TokenType::For => "For",
        TokenType::Do => "Do",
        TokenType::While => "While",
        TokenType::Loop => "Loop",
        TokenType::Break => "Break",
        TokenType::To => "To",
        TokenType::Etc => "...",
        TokenType::Match => "Match",
        TokenType::In => "In",
        TokenType::With => "With",
        TokenType::BoolType => "BoolType",
        TokenType::UInt8Type => "UInt8Type",
        TokenType::UInt16Type => "UInt16Type",
        TokenType::UInt32Type => "UInt32Type",
        TokenType::UInt64Type => "UInt64Type",
        TokenType::UInt128Type => "UInt128Type",
        TokenType::Int8Type => "Int8Type",
        TokenType::Int16Type => "Int16Type",
        TokenType::Int32Type => "Int32Type",
        TokenType::Int64Type => "Int64Type",
        TokenType::Int128Type => "Int128Type",
        TokenType::Float32Type => "Float32Type",
        TokenType::Float64Type => "Float64Type",
        TokenType::CharType => "CharType",
        TokenType::StringType => "StringType",
        TokenType::Array => "Array",
        TokenType::Table => "Table",
        TokenType::Stack => "Stack",
        TokenType::Queue => "Queue",
        TokenType::Set => "Set",
        TokenType::List => "List",
        TokenType::TwoList => "TwoList",
        TokenType::Heap => "Heap",
        TokenType::Graph => "Graph",
        TokenType::Tree => "Tree",
        TokenType::Enum => "Enum",
        TokenType::Type => "Type",
        TokenType::Struct => "Struct",
        TokenType::Return => "Return",
        TokenType::And => "And",
        TokenType::Or => "Or",
        TokenType::Not => "Not",
        TokenType::Xor => "Xor",
        TokenType::True => "True",
        TokenType::False => "False",
        TokenType::Nil => "Nil",
    }
    .to_owned()
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token_new() {
        let token = Token::new(TokenType::Int32Type, "int32".to_owned());
        assert_eq!(token.Type, TokenType::Int32Type);
        assert_eq!(token.type_literal, "Int32Type".to_owned());
        assert_eq!(token.value, "int32".to_owned());
    }

    #[test]
    fn test_token_new_type() {
        let mut token = Token::new(TokenType::Int32Type, "int32".to_owned());
        token.new_type(TokenType::Int64Type);
        assert_eq!(token.Type, TokenType::Int64Type);
        assert_eq!(token.type_literal, "Int64Type".to_owned());
    }

    #[test]
    fn test_token_reevaluate_token() {
        let mut token = Token::new(TokenType::Int32Type, "int32".to_owned());
        let value = "int64".to_owned();
        token.reevaluate_token(value.clone());
        assert_eq!(token.Type, TokenType::Int64Type);
        assert_eq!(token.type_literal, "Int64Type".to_owned());
        assert_eq!(token.value, value);
    }
}
