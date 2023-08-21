use monkey::lexer::Lexer;
use monkey::token::TokenKind;

fn main() {
    let mut rl = rustyline::DefaultEditor::new().expect("failed to open repel");

    println!("monkey programming language");

    loop {
        match rl.readline("> ") {
            Ok(line) => {
                let mut lexer = Lexer::new(line);

                let mut t = lexer.next_token();

                while t.kind != TokenKind::Eof {
                    println!("Token( {:?}, \"{}\" )", t.kind, lexer.extract(t.span));
                    t = lexer.next_token();
                }
            }

            Err(rustyline::error::ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            }

            Err(rustyline::error::ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            }

            Err(_) => {
                break;
            }
        }
    }
}
