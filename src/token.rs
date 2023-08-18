#[derive(Debug, Clone, Copy)]
pub enum Token {
    Illegal { pos: usize },
    Eof { pos: usize },

    Ident { pos: usize, len: usize },
    Int { pos: usize, len: usize },

    Assign { pos: usize },
    Plus { pos: usize },

    Comma { pos: usize },
    Semicolon { pos: usize },

    LParan { pos: usize },
    RParan { pos: usize },
    LBrace { pos: usize },
    RBrace { pos: usize },

    Let { pos: usize, len: usize },
    Fn { pos: usize, len: usize },
}
