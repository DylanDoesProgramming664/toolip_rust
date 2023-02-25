#![allow(dead_code, non_snake_case)]
use crate::lexer::Lexer;
use std::process::exit;

pub const KEYWORDS: &[&str] = &[
    "illegal",
    "global",
    "const",
    "static",
    "jit",
    "unsafe",
    "coroutine",
    "func",
    "this",
    "struct",
    "self",
    "end",
    "if",
    "else",
    "elseif",
    "then",
    "for",
    "do",
    "while",
    "loop",
    "break",
    "given",
    "when",
    "in",
    "with",
    "bool",
    "uint8",
    "uint16",
    "uint32",
    "uint64",
    "uint128",
    "int8",
    "int16",
    "int32",
    "int64",
    "int128",
    "flt32",
    "flt64",
    "char",
    "string",
    "array",
    "table",
    "stack",
    "queue",
    "set",
    "list",
    "twolist",
    "heap",
    "tree",
    "enum",
    "type",
    "return",
    "and",
    "or",
    "not",
    "xor",
    "true",
    "false",
    "nil",
];

pub const SYMBOLS: &[&str] = &[
    "illegal", "==", "=>", "=", "++", "+=", "+", "--", "-=", "->", "-", "**=", "**", "*=", "*",
    "//=", "//", "/=", "/", "%=", "%", "!=", "!", "~", "&=", "&", "|=", "|", "^=", "^", "??=",
    "??", "?", "...=", "...", "..=", "..", ".", "<<=", "<<", "<=", "<", ">>=", ">>", ">=", ">",
    "'", "\"", "`", ";", "(", ")", "[", "]", "{", "}", "#", ",", ":", "@", "$",
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(usize)]
pub enum KeywordKind {
    Illegal = 0,
    Global = 1,
    Const = 2,
    Static = 3,
    Jit = 4,
    Unsafe = 5,
    Coroutine = 6,
    Func = 7,
    This = 8,
    Struct = 9,
    SelF = 10,
    End = 11,
    If = 12,
    Else = 13,
    ElseIf = 14,
    Then = 15,
    For = 16,
    Do = 17,
    While = 18,
    Loop = 19,
    Break = 20,
    Given = 21,
    When = 22,
    In = 23,
    With = 24,
    Bool = 25,
    Uint8 = 26,
    Uint16 = 27,
    Uint32 = 28,
    Uint64 = 29,
    Uint128 = 30,
    Int8 = 31,
    Int16 = 32,
    Int32 = 33,
    Int64 = 34,
    Int128 = 35,
    Flt32 = 36,
    Flt64 = 37,
    Char = 38,
    String = 39,
    Array = 40,
    Table = 41,
    Stack = 42,
    Queue = 43,
    Set = 44,
    List = 45,
    Twolist = 46,
    Heap = 47,
    Tree = 48,
    Enum = 49,
    Type = 50,
    Return = 51,
    And = 52,
    Or = 53,
    Not = 54,
    Xor = 55,
    True = 56,
    False = 57,
    Nil = 58,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(usize)]
pub enum SymbolKind {
    Illegal = 0,
    Equals = 1,
    FatArrow = 2,
    Assign = 3,
    Increment = 4,
    PlusAssign = 5,
    Plus = 6,
    Decrement = 7,
    MinusAssign = 8,
    ThinArrow = 9,
    Minus = 10,
    ExponentAssign = 11,
    Exponent = 12,
    MultiplyAssign = 13,
    Multiply = 14,
    FloorDivideAssign = 15,
    FloorDivide = 16,
    DivideAssign = 17,
    Divide = 18,
    ModuloAssign = 19,
    Modulo = 20,
    BoolNotEQ = 21,
    BoolNot = 22,
    BitNot = 23,
    BitAndAssign = 24,
    BitAnd = 25,
    BitOrAssign = 26,
    BitOr = 27,
    BitXorAssign = 28,
    BitXor = 29,
    NilCoalesceAssign = 30,
    NilCoalesce = 31,
    Ternary = 32,
    EtcEQ = 33,
    Etc = 34,
    ConcatAssign = 35,
    Concat = 36,
    Dot = 37,
    BitshiftLeftAssign = 38,
    BitshiftLeft = 39,
    LessThanEquals = 40,
    LessThan = 41,
    BitshiftRightAssign = 42,
    BitshiftRight = 43,
    GreaterThanEquals = 44,
    GreaterThan = 45,
    SingleQuote = 46,
    DoubleQuote = 47,
    Grave = 48,
    Semicolon = 49,
    LeftParen = 50,
    RightParen = 51,
    LeftBracket = 52,
    RightBracket = 53,
    LeftBrace = 54,
    RightBrace = 55,
    Hash = 56,
    Comma = 57,
    Colon = 58,
    AtSign = 59,
    DollarSign = 60,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub Type: TokenType,
    pub type_literal: String,
    pub value: String,
}

impl Token {
    pub fn new(tok_type: TokenType, value: String) -> Self {
        let tok_lit = get_type_literal(&tok_type);
        Self {
            Type: tok_type,
            type_literal: tok_lit,
            value,
        }
    }

    pub fn new_type(&mut self, tok_type: TokenType) {
        self.type_literal = get_type_literal(&tok_type);
        self.Type = tok_type;
    }

    pub fn reevaluate_token(&mut self, tok_value: &str) {
        let input = tok_value.chars().collect::<Vec<char>>();
        let mut sub_lexer = Lexer::new(input);
        let tok = sub_lexer.next_token();
        self.type_literal = get_type_literal(&tok.Type);
        self.Type = tok.Type;
        self.value = tok.value;
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(usize)]
pub enum TokenType {
    Empty = 0,
    Illegal = 1,
    NewLine = 2,
    Eof = 3,

    /* Identifiers & Literals */
    Identifier(String) = 4,
    BoolVal(bool) = 5,
    UInt8Val(u8) = 6,
    UInt16Val(u16) = 7,
    UInt32Val(u32) = 8,
    UInt64Val(u64) = 9,
    UInt128Val(u128) = 10,
    Int8Val(i8) = 11,
    Int16Val(i16) = 12,
    Int32Val(i32) = 13,
    Int64Val(i64) = 14,
    Int128Val(i128) = 15,
    Float32Val(f32) = 16,
    Float64Val(f64) = 17,
    CharVal(char) = 18,
    StringVal(String) = 19,

    Symbol(SymbolKind) = 20,
    Keyword(KeywordKind) = 21,
}

pub fn get_type_literal(tok_type: &TokenType) -> String {
    match tok_type {
        TokenType::Empty => "<None>",
        TokenType::Illegal => "<Illegal>",
        TokenType::NewLine => "<NewLine>",
        TokenType::Eof => "<Eof>",
        TokenType::Identifier(_) => "Identifier",
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
        TokenType::Symbol(x) => SYMBOLS[*x as usize],
        TokenType::Keyword(x) => get_keyword_literal(x),
    }
    .to_owned()
}

const fn get_keyword_literal(a: &KeywordKind) -> &str {
    match *a {
        KeywordKind::Illegal => "KW_Illegal",
        KeywordKind::Global => "KW_Global",
        KeywordKind::Const => "KW_Const",
        KeywordKind::Static => "KW_Static",
        KeywordKind::Jit => "KW_Jit",
        KeywordKind::Unsafe => "KW_Unsafe",
        KeywordKind::Coroutine => "KW_Coroutine",
        KeywordKind::Func => "KW_Func",
        KeywordKind::This => "KW_This",
        KeywordKind::Struct => "KW_Struct",
        KeywordKind::SelF => "KW_Self",
        KeywordKind::End => "KW_End",
        KeywordKind::If => "KW_If",
        KeywordKind::Else => "KW_Else",
        KeywordKind::ElseIf => "KW_ElseIf",
        KeywordKind::Then => "KW_Then",
        KeywordKind::For => "KW_For",
        KeywordKind::Do => "KW_Do",
        KeywordKind::While => "KW_While",
        KeywordKind::Loop => "KW_Loop",
        KeywordKind::Break => "KW_Break",
        KeywordKind::Given => "KW_Given",
        KeywordKind::When => "KW_When",
        KeywordKind::In => "KW_In",
        KeywordKind::With => "KW_With",
        KeywordKind::Bool => "KW_Bool",
        KeywordKind::Uint8 => "KW_Uint8",
        KeywordKind::Uint16 => "KW_Uint16",
        KeywordKind::Uint32 => "KW_Uint32",
        KeywordKind::Uint64 => "KW_Uint64",
        KeywordKind::Uint128 => "KW_Uint128",
        KeywordKind::Int8 => "KW_Int8",
        KeywordKind::Int16 => "KW_Int16",
        KeywordKind::Int32 => "KW_Int32",
        KeywordKind::Int64 => "KW_Int64",
        KeywordKind::Int128 => "KW_Int128",
        KeywordKind::Flt32 => "KW_Flt32",
        KeywordKind::Flt64 => "KW_Flt64",
        KeywordKind::Char => "KW_Char",
        KeywordKind::String => "KW_String",
        KeywordKind::Array => "KW_Array",
        KeywordKind::Table => "KW_Table",
        KeywordKind::Stack => "KW_Stack",
        KeywordKind::Queue => "KW_Queue",
        KeywordKind::Set => "KW_Set",
        KeywordKind::List => "KW_List",
        KeywordKind::Twolist => "KW_Twolist",
        KeywordKind::Heap => "KW_Heap",
        KeywordKind::Tree => "KW_Tree",
        KeywordKind::Enum => "KW_Enum",
        KeywordKind::Type => "KW_Type",
        KeywordKind::Return => "KW_Return",
        KeywordKind::And => "KW_And",
        KeywordKind::Or => "KW_Or",
        KeywordKind::Not => "KW_Not",
        KeywordKind::Xor => "KW_Xor",
        KeywordKind::True => "KW_True",
        KeywordKind::False => "KW_False",
        KeywordKind::Nil => "KW_Nil",
    }
}

pub fn match_keyword_to_index(index: usize) -> KeywordKind {
    match index {
        0 => KeywordKind::Illegal,
        1 => KeywordKind::Global,
        2 => KeywordKind::Const,
        3 => KeywordKind::Static,
        4 => KeywordKind::Jit,
        5 => KeywordKind::Unsafe,
        6 => KeywordKind::Coroutine,
        7 => KeywordKind::Func,
        8 => KeywordKind::This,
        9 => KeywordKind::Struct,
        10 => KeywordKind::SelF,
        11 => KeywordKind::End,
        12 => KeywordKind::If,
        13 => KeywordKind::Else,
        14 => KeywordKind::ElseIf,
        15 => KeywordKind::Then,
        16 => KeywordKind::For,
        17 => KeywordKind::Do,
        18 => KeywordKind::While,
        19 => KeywordKind::Loop,
        20 => KeywordKind::Break,
        21 => KeywordKind::Given,
        22 => KeywordKind::When,
        23 => KeywordKind::In,
        24 => KeywordKind::With,
        25 => KeywordKind::Bool,
        26 => KeywordKind::Uint8,
        27 => KeywordKind::Uint16,
        28 => KeywordKind::Uint32,
        29 => KeywordKind::Uint64,
        30 => KeywordKind::Uint128,
        31 => KeywordKind::Int8,
        32 => KeywordKind::Int16,
        33 => KeywordKind::Int32,
        34 => KeywordKind::Int64,
        35 => KeywordKind::Int128,
        36 => KeywordKind::Flt32,
        37 => KeywordKind::Flt64,
        38 => KeywordKind::Char,
        39 => KeywordKind::String,
        40 => KeywordKind::Array,
        41 => KeywordKind::Table,
        42 => KeywordKind::Stack,
        43 => KeywordKind::Queue,
        44 => KeywordKind::Set,
        45 => KeywordKind::List,
        46 => KeywordKind::Twolist,
        47 => KeywordKind::Heap,
        48 => KeywordKind::Tree,
        49 => KeywordKind::Enum,
        50 => KeywordKind::Type,
        51 => KeywordKind::Return,
        52 => KeywordKind::And,
        53 => KeywordKind::Or,
        54 => KeywordKind::Not,
        55 => KeywordKind::Xor,
        56 => KeywordKind::True,
        57 => KeywordKind::False,
        58 => KeywordKind::Nil,
        _ => {
            println!("{index} is out of bounds.");
            exit(1);
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_len_keywords() {
        assert_eq!(KEYWORDS.len(), 59);
    }

    #[test]
    fn test_len_symbols() {
        assert_eq!(SYMBOLS.len(), 61);
    }

    #[test]
    fn test_token_new() {
        let token = Token::new(TokenType::Keyword(KeywordKind::Int32), "int32".to_owned());
        assert_eq!(token.Type, TokenType::Keyword(KeywordKind::Int32));
        assert_eq!(token.type_literal, "KW_Int32".to_owned());
        assert_eq!(token.value, "int32".to_owned());
    }

    #[test]
    fn test_token_new_type() {
        let mut token = Token::new(TokenType::Keyword(KeywordKind::Int32), "int32".to_owned());
        token.new_type(TokenType::Keyword(KeywordKind::Int64));
        assert_eq!(token.Type, TokenType::Keyword(KeywordKind::Int64));
        assert_eq!(token.type_literal, "KW_Int64".to_owned());
    }

    #[test]
    fn test_token_reevaluate_token() {
        let mut token = Token::new(TokenType::Keyword(KeywordKind::Int32), "int32".to_owned());
        let value = "int64".to_owned();
        token.reevaluate_token(&value);
        assert_eq!(token.Type, TokenType::Keyword(KeywordKind::Int64));
        assert_eq!(token.type_literal, "KW_Int64".to_owned());
        assert_eq!(token.value, value);
    }
}
