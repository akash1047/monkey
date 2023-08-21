use monkey::lexer::Lexer;
use monkey::token::TokenKind;

#[test]
fn keyword_space_seperated() {
    let input = "fn let\nreturn\rif\telse\t\ntrue \n\r\tfalse".to_string();

    let mut lexer = Lexer::new(input);

    let mut check = |k, v| {
        let t = lexer.next_token();
        assert_eq!(k, t.kind);
        assert_eq!(v, lexer.extract(t.span));
    };

    check(TokenKind::Fn, "fn");
    check(TokenKind::Let, "let");
    check(TokenKind::Return, "return");
    check(TokenKind::If, "if");
    check(TokenKind::Else, "else");
    check(TokenKind::True, "true");
    check(TokenKind::False, "false");
    check(TokenKind::Eof,  "");
}
