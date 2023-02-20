#![allow(dead_code, non_snake_case)]
use crate::lexer::Lexer;
#[derive(Clone, PartialEq, Debug)]
pub struct Token {
    pub Type: Type,
    pub type_literal: String,
    pub value: String,
}

impl Token {
    pub fn new(tok_type: Type, value: String) -> Self {
        let tok_lit = get_type_literal(&tok_type);
        Self {
            Type: tok_type,
            type_literal: tok_lit,
            value,
        }
    }

    pub fn new_type(&mut self, tok_type: Type) {
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
pub enum Type {
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

pub fn get_type_literal(tok_type: &Type) -> String {
    match tok_type {
        Type::None => "<None>",
        Type::Illegal => "<Illegal>",
        Type::NewLine => "<NewLine>",
        Type::Eof => "<Eof>",
        Type::Identifier => "Identifier",
        Type::BoolVal(_) => "BoolVal",
        Type::UInt8Val(_) => "UInt8Val",
        Type::UInt16Val(_) => "UInt16Val",
        Type::UInt32Val(_) => "UInt32Val",
        Type::UInt64Val(_) => "UInt64Val",
        Type::UInt128Val(_) => "UInt128Val",
        Type::Int8Val(_) => "Int8Val",
        Type::Int16Val(_) => "Int16Val",
        Type::Int32Val(_) => "Int32Val",
        Type::Int64Val(_) => "Int64Val",
        Type::Int128Val(_) => "Int128Val",
        Type::Float32Val(_) => "Float32Val",
        Type::Float64Val(_) => "Float64Val",
        Type::CharVal(_) => "CharVal",
        Type::StringVal(_) => "StringVal",
        Type::Equals => "==",
        Type::Assign => "=",
        Type::Increment => "++",
        Type::Decrement => "--",
        Type::LShiftEQ => "<<=",
        Type::RShiftEQ => ">>=",
        Type::PlusEQ => "+=",
        Type::MinusEQ => "-=",
        Type::ExpoEQ => "**=",
        Type::MultEQ => "*=",
        Type::FDivEQ => "//=",
        Type::DivEQ => "/=",
        Type::ModEQ => "%=",
        Type::ConcatEQ => "..=",
        Type::CoalesceEQ => "??=",
        Type::LessThanEQ => "<=",
        Type::GreaterThanEQ => ">=",
        Type::BoolNotEQ => "!=",
        Type::BitAndEQ => "&=",
        Type::BitOrEQ => "|=",
        Type::BitXorEQ => "^=",
        Type::LShift => "<<",
        Type::RShift => ">>",
        Type::Plus => "+",
        Type::Minus => "-",
        Type::Expo => "**",
        Type::Mult => "*",
        Type::FDiv => "//",
        Type::Div => "/",
        Type::Mod => "%",
        Type::Concat => "..",
        Type::Coalesce => "??",
        Type::Ternary => "?",
        Type::LessThan => "<",
        Type::GreaterThan => ">",
        Type::BoolNot => "!",
        Type::BitNot => "~",
        Type::BitAnd => "&",
        Type::BitOr => "|",
        Type::BitXor => "^",
        Type::Hash => "#",
        Type::Semicolon => ";",
        Type::Comma => ",",
        Type::Dot => ".",
        Type::Colon => ":",
        Type::ThinArrow => "->",
        Type::ThickArrow => "=>",
        Type::AtSign => "@",
        Type::Dollar => "$",
        Type::LParen => "(",
        Type::RParen => ")",
        Type::LBrace => "{",
        Type::RBrace => "}",
        Type::LBracket => "[",
        Type::RBracket => "]",
        Type::Global => "Global",
        Type::Const => "Const",
        Type::Static => "Static",
        Type::Jit => "Jit",
        Type::Unsafe => "Unsafe",
        Type::Coroutine => "Coroutine",
        Type::Func => "Func",
        Type::This => "This",
        Type::End => "End",
        Type::Slf => "Self",
        Type::If => "If",
        Type::Then => "Then",
        Type::Else => "Else",
        Type::ElseIf => "ElseIf",
        Type::For => "For",
        Type::Do => "Do",
        Type::While => "While",
        Type::Loop => "Loop",
        Type::Break => "Break",
        Type::To => "To",
        Type::Etc => "...",
        Type::Match => "Match",
        Type::In => "In",
        Type::With => "With",
        Type::BoolType => "BoolType",
        Type::UInt8Type => "UInt8Type",
        Type::UInt16Type => "UInt16Type",
        Type::UInt32Type => "UInt32Type",
        Type::UInt64Type => "UInt64Type",
        Type::UInt128Type => "UInt128Type",
        Type::Int8Type => "Int8Type",
        Type::Int16Type => "Int16Type",
        Type::Int32Type => "Int32Type",
        Type::Int64Type => "Int64Type",
        Type::Int128Type => "Int128Type",
        Type::Float32Type => "Float32Type",
        Type::Float64Type => "Float64Type",
        Type::CharType => "CharType",
        Type::StringType => "StringType",
        Type::Array => "Array",
        Type::Table => "Table",
        Type::Stack => "Stack",
        Type::Queue => "Queue",
        Type::Set => "Set",
        Type::List => "List",
        Type::TwoList => "TwoList",
        Type::Heap => "Heap",
        Type::Graph => "Graph",
        Type::Tree => "Tree",
        Type::Enum => "Enum",
        Type::Type => "Type",
        Type::Struct => "Struct",
        Type::Return => "Return",
        Type::And => "And",
        Type::Or => "Or",
        Type::Not => "Not",
        Type::Xor => "Xor",
        Type::True => "True",
        Type::False => "False",
        Type::Nil => "Nil",
    }
    .to_owned()
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token_new() {
        let token = Token::new(Type::Int32Type, "int32".to_owned());
        assert_eq!(token.Type, Type::Int32Type);
        assert_eq!(token.type_literal, "Int32Type".to_owned());
        assert_eq!(token.value, "int32".to_owned());
    }

    #[test]
    fn test_token_new_type() {
        let mut token = Token::new(Type::Int32Type, "int32".to_owned());
        token.new_type(Type::Int64Type);
        assert_eq!(token.Type, Type::Int64Type);
        assert_eq!(token.type_literal, "Int64Type".to_owned());
    }

    #[test]
    fn test_token_reevaluate_token() {
        let mut token = Token::new(Type::Int32Type, "int32".to_owned());
        let value = "int64".to_owned();
        token.reevaluate_token(value.clone());
        assert_eq!(token.Type, Type::Int64Type);
        assert_eq!(token.type_literal, "Int64Type".to_owned());
        assert_eq!(token.value, value);
    }
}
