#[derive(Debug)]
pub struct Lexer {
    input: String,
    position: usize,
    read_position: usize,
    ch: u8,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        let mut lexer = Self {
            input,
            position: 0,
            read_position: 0,
            ch: 0,
        };

        lexer.next_char();

        lexer
    }

    pub fn next_char(&mut self) {
        if let Some(&ch) = self.input.as_bytes().get(self.read_position) {
            self.ch = ch;
            self.position = self.read_position;
        } else {
            self.ch = 0;
            self.position = self.input.len();
        }
        self.read_position += 1;
    }

    pub fn peak(&self) -> u8 {
        match self.input.as_bytes().get(self.read_position) {
            Some(&ch) => ch,
            None => 0,
        }
    }

    pub fn look_ahead(&self, nth: usize) -> u8 {
        match self.input.as_bytes().get(self.position + nth) {
            Some(&ch) => ch,
            None => 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn state_of_lexer_on_calling_next_char() {
        let input = "int x;".to_string();
        let mut lexer = Lexer::new(input);

        assert_eq!(0, lexer.position);
        assert_eq!(1, lexer.read_position);
        assert_eq!(b'i', lexer.ch);

        lexer.next_char();
        assert_eq!(1, lexer.position);
        assert_eq!(2, lexer.read_position);
        assert_eq!(b'n', lexer.ch);

        lexer.next_char();
        assert_eq!(2, lexer.position);
        assert_eq!(3, lexer.read_position);
        assert_eq!(b't', lexer.ch);

        lexer.next_char();
        assert_eq!(3, lexer.position);
        assert_eq!(4, lexer.read_position);
        assert_eq!(b' ', lexer.ch);

        lexer.next_char();
        assert_eq!(4, lexer.position);
        assert_eq!(5, lexer.read_position);
        assert_eq!(b'x', lexer.ch);

        lexer.next_char();
        assert_eq!(5, lexer.position);
        assert_eq!(6, lexer.read_position);
        assert_eq!(b';', lexer.ch);

        lexer.next_char();
        assert_eq!(6, lexer.position);
        assert_eq!(7, lexer.read_position);
        assert_eq!(0, lexer.ch);

        lexer.next_char();
        assert_eq!(6, lexer.position);
        assert_eq!(8, lexer.read_position);
        assert_eq!(0, lexer.ch);

        lexer.next_char();
        assert_eq!(6, lexer.position);
        assert_eq!(9, lexer.read_position);
        assert_eq!(0, lexer.ch);

        lexer.next_char();
        assert_eq!(6, lexer.position);
        assert_eq!(10, lexer.read_position);
        assert_eq!(0, lexer.ch);
    }
}
