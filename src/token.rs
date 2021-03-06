use crate::id::Id;

#[derive(Debug, PartialEq, Clone)]
pub enum TokenKind {
    Number(i64),
    Add,
    Sub,
    Asterisk,
    Div,
    Lparen,
    Rparen,
    Lbrace,
    Rbrace,
    EOF,
    Equal,
    NotEqual,
    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,
    Ident(Id),
    Semicolon,
    Assign,
    Return,
    If,
    Else,
    While,
    For,
    Comma,
    Int,
    Ampersand,
    SizeOf,
    Lbracket,
    Rbracket,
    Char,
    String(Id),
    Or,
    Xor,
    BitNot,
    Mod,
    Shl,
    Shr,
    Hash,
    NewLine,
    Struct,
    Dot,
    Arrow,
    Void,
    Typedef,
    Short,
    Long,
    Enum,
    Static,
    Switch,
    Case,
    Colon,
    Break,
    Extern,
    Continue,
    Default,
    Const,
    Float,
    FloatNum(f32),
    Double,
    DoubleNum(f64),
    Goto,
}

impl ToString for TokenKind {
    fn to_string(&self) -> String {
        String::from(match self {
            TokenKind::Number(_) => "number",
            TokenKind::Add => "+",
            TokenKind::Sub => "-",
            TokenKind::Asterisk => "*",
            TokenKind::Div => "/",
            TokenKind::Lparen => "(",
            TokenKind::Rparen => ")",
            TokenKind::Lbrace => "{",
            TokenKind::Rbrace => "}",
            TokenKind::EOF => "EOF",
            TokenKind::Equal => "==",
            TokenKind::NotEqual => "!=",
            TokenKind::LessThan => "<",
            TokenKind::LessThanOrEqual => "<=",
            TokenKind::GreaterThan => ">",
            TokenKind::GreaterThanOrEqual => ">=",
            TokenKind::Ident(_) => "identifier",
            TokenKind::Semicolon => ";",
            TokenKind::Assign => "=",
            TokenKind::Return => "return",
            TokenKind::If => "if",
            TokenKind::Else => "else",
            TokenKind::While => "while",
            TokenKind::For => "for",
            TokenKind::Comma => ",",
            TokenKind::Int => "int",
            TokenKind::Ampersand => "&",
            TokenKind::SizeOf => "sizeof",
            TokenKind::Lbracket => "[",
            TokenKind::Rbracket => "]",
            TokenKind::Char => "char",
            TokenKind::String(_) => "string",
            TokenKind::Or => "|",
            TokenKind::Xor => "^",
            TokenKind::BitNot => "~",
            TokenKind::Mod => "%",
            TokenKind::Shl => "<<",
            TokenKind::Shr => ">>",
            TokenKind::Hash => "#",
            TokenKind::NewLine => "\\n",
            TokenKind::Struct => "struct",
            TokenKind::Dot => ".",
            TokenKind::Arrow => "->",
            TokenKind::Void => "void",
            TokenKind::Typedef => "typedef",
            TokenKind::Short => "short",
            TokenKind::Long => "long",
            TokenKind::Enum => "enum",
            TokenKind::Static => "static",
            TokenKind::Switch => "switch",
            TokenKind::Case => "case",
            TokenKind::Colon => ":",
            TokenKind::Break => "break",
            TokenKind::Extern => "extern",
            TokenKind::Continue => "continue",
            TokenKind::Default => "default",
            TokenKind::Const => "const",
            TokenKind::Float => "float",
            TokenKind::FloatNum(_) => "float",
            TokenKind::Double => "double",
            TokenKind::DoubleNum(_) => "double",
            TokenKind::Goto => "goto",
        })
    }
}

#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub start_line: usize,
    pub start_col: usize,
    pub end_line: usize,
    pub end_col: usize,
}
