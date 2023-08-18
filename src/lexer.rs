use crate::token::*;

const KEYWORDS: [(&'static str, TokenKind); 3] = [
    ("fn", TokenKind::Fn),
    ("let", TokenKind::Let),
    ("return", TokenKind::Return),
];

pub struct Lexer {
    input: String,
    pos: usize,
    read_pos: usize,
    ch: u8,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        let mut l = Self {
            input,
            pos: 0,
            read_pos: 0,
            ch: 0,
        };

        l.read_char();

        l
    }

    pub fn read_char(&mut self) {
        if let Some(&ch) = self.input.as_bytes().get(self.read_pos) {
            self.ch = ch;
            self.pos = self.read_pos;
        } else {
            self.ch = 0;
            self.pos = self.input.len();
        }

        self.read_pos += 1;
    }
    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let span = |len| Span::new(self.pos, len);

        let mut t = Token {
            kind: TokenKind::Illegal,
            span: span(1),
        };

        match &self.ch {
            b'=' => t.kind = TokenKind::Assign,
            b'+' => t.kind = TokenKind::Plus,
            b'-' => t.kind = TokenKind::Minus,
            b',' => t.kind = TokenKind::Comma,
            b';' => t.kind = TokenKind::Semicolon,
            b'(' => t.kind = TokenKind::LParan,
            b')' => t.kind = TokenKind::RParan,
            b'{' => t.kind = TokenKind::LBrace,
            b'}' => t.kind = TokenKind::RBrace,
            b'"' => {
                return self.read_string();
            }
            0 => {
                t.kind = TokenKind::Eof;
                t.span = span(0);
            }

            _ => {
                if self.ch.is_ascii_alphabetic() || self.ch == b'_' {
                    t.span = self.read_identifier();
                    t.kind = lookup_ident(self.extract(t.span));

                    return t;
                } else if self.ch.is_ascii_digit() {
                    return self.read_number();
                }
            }
        }

        self.read_char();

        t
    }

    fn read_string(&mut self) -> Token {
        let pos = self.pos;

        // as self.ch is " is checked in next_token() functions
        // we advance the cursor
        self.read_char();

        while self.ch != b'"' {
            self.read_char();
        }

        if self.ch == b'"' {
            self.read_char();
        }

        Token {
            kind: TokenKind::String,
            span: Span::new(pos, self.pos - pos),
        }
    }

    fn read_identifier(&mut self) -> Span {
        let pos = self.pos;

        // as the self.ch is alphabet or underscore is checked in next_token() function
        // we advance the cursor
        self.read_char();

        while self.ch.is_ascii_alphanumeric() || self.ch == b'_' {
            self.read_char();
        }

        Span::new(pos, self.pos - pos)
    }

    fn read_number(&mut self) -> Token {
        let pos = self.pos;
        let mut k = TokenKind::Int;

        // as self.ch isAsciiDigit is checked in next_token() function
        // we advance the cursor
        self.read_char();

        while self.ch.is_ascii_digit() {
            self.read_char();
        }

        // check if it has fraction
        if self.ch == b'.' {
            k = TokenKind::Float;
            self.read_char();

            while self.ch.is_ascii_digit() {
                self.read_char();
            }
        }

        Token {
            kind: k,
            span: Span::new(pos, self.pos - pos),
        }
    }

    pub fn extract(&self, s: Span) -> &'_ str {
        &self.input[s.pos()..s.pos() + s.len()]
    }

    pub fn skip_whitespace(&mut self) {
        while self.ch.is_ascii_whitespace() {
            self.read_char();
        }
    }
}

fn lookup_ident(s: &str) -> TokenKind {
    for (l, t) in KEYWORDS {
        if l == s {
            return t;
        }
    }

    TokenKind::Ident
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn function() {
        let input = r#"let add = fn(x, y) { x + y };

let sub = fn(x, y) {
    return x - y;
};

fn greet() {
    println("hello");
}
"#;

        let mut l = Lexer::new(input.to_string());

        let mut check = |ty, val| {
            let t = l.next_token();
            assert_eq!(ty, t.kind);
            assert_eq!(val, l.extract(t.span));
        };

        check(TokenKind::Let, "let");
        check(TokenKind::Ident, "add");
        check(TokenKind::Assign, "=");
        check(TokenKind::Fn, "fn");
        check(TokenKind::LParan, "(");
        check(TokenKind::Ident, "x");
        check(TokenKind::Comma, ",");
        check(TokenKind::Ident, "y");
        check(TokenKind::RParan, ")");
        check(TokenKind::LBrace, "{");
        check(TokenKind::Ident, "x");
        check(TokenKind::Plus, "+");
        check(TokenKind::Ident, "y");
        check(TokenKind::RBrace, "}");
        check(TokenKind::Semicolon, ";");

        check(TokenKind::Let, "let");
        check(TokenKind::Ident, "sub");
        check(TokenKind::Assign, "=");
        check(TokenKind::Fn, "fn");
        check(TokenKind::LParan, "(");
        check(TokenKind::Ident, "x");
        check(TokenKind::Comma, ",");
        check(TokenKind::Ident, "y");
        check(TokenKind::RParan, ")");
        check(TokenKind::LBrace, "{");
        check(TokenKind::Return, "return");
        check(TokenKind::Ident, "x");
        check(TokenKind::Minus, "-");
        check(TokenKind::Ident, "y");
        check(TokenKind::Semicolon, ";");
        check(TokenKind::RBrace, "}");
        check(TokenKind::Semicolon, ";");

        check(TokenKind::Fn, "fn");
        check(TokenKind::Ident, "greet");
        check(TokenKind::LParan, "(");
        check(TokenKind::RParan, ")");
        check(TokenKind::LBrace, "{");
        check(TokenKind::Ident, "println");
        check(TokenKind::LParan, "(");
        check(TokenKind::String, "\"hello\"");
        check(TokenKind::RParan, ")");
        check(TokenKind::Semicolon, ";");
        check(TokenKind::RBrace, "}");

        check(TokenKind::Eof, "");
    }

    #[test]
    fn variables() {
        let input = r#"let i = 12;
let pi;
pi = 3.14;

let message = "i is an int, pi is a float";

4.
42.0
123
"ok"
bobby = "bobby poddar";
"#;

        let mut l = Lexer::new(input.to_string());

        let mut check = |ty, val| {
            let t = l.next_token();
            assert_eq!(ty, t.kind);
            assert_eq!(val, l.extract(t.span));
        };

        check(TokenKind::Let, "let");
        check(TokenKind::Ident, "i");
        check(TokenKind::Assign, "=");
        check(TokenKind::Int, "12");
        check(TokenKind::Semicolon, ";");

        check(TokenKind::Let, "let");
        check(TokenKind::Ident, "pi");
        check(TokenKind::Semicolon, ";");

        check(TokenKind::Ident, "pi");
        check(TokenKind::Assign, "=");
        check(TokenKind::Float, "3.14");
        check(TokenKind::Semicolon, ";");

        check(TokenKind::Let, "let");
        check(TokenKind::Ident, "message");
        check(TokenKind::Assign, "=");
        check(TokenKind::String, "\"i is an int, pi is a float\"");
        check(TokenKind::Semicolon, ";");

        check(TokenKind::Float, "4.");
        check(TokenKind::Float, "42.0");
        check(TokenKind::Int, "123");
        check(TokenKind::String, "\"ok\"");

        check(TokenKind::Ident, "bobby");
        check(TokenKind::Assign, "=");
        check(TokenKind::String, "\"bobby poddar\"");
        check(TokenKind::Semicolon, ";");

        check(TokenKind::Eof, "");
    }
}
