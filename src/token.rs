#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TokenKind {
    Illegal,
    Eof,

    Identifier,
    IntLiteral,

    Assign,
    Plus,

    Comma,
    Semicolon,

    LeftParenthesis,
    RightParenthesis,
    LeftBrace,
    RightBrace,

    FnKeyword,
    LetKeyword,
}


#[derive(Debug, Clone, Copy)]
pub struct Token<'a> {
    kind: TokenKind,
    value: &'a str,
}

