#[derive(Debug, Clone, Copy)]
pub struct Span(usize, usize);

impl Span {
    pub fn new(pos: usize, len: usize) -> Self {
        Self(pos, len)
    }

    pub fn pos(&self) -> usize {
        self.0
    }

    pub fn len(&self) -> usize {
        self.1
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TokenKind {
    Illegal,
    Eof,

    Ident,
    Int,
    Float,
    String,

    Assign,
    Plus,
    Minus,

    Comma,
    Semicolon,

    LParan,
    RParan,
    LBrace,
    RBrace,

    Let,
    Fn,
    Return,
}

#[derive(Debug, Clone, Copy)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}
