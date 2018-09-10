use tini::lexer::*;
use tini::token::*;

#[test]
fn lexer_test() {
    use crate::TokenType::*;

    let input = r#"
    (if (= (+ 1 2 3 4 5) 15)
        (print "Yeah")
        (print "Nope"))
    "#;

    let tokens = [
        LeftBracket,
        If,
        LeftBracket,
        Identifier("=".to_string()),
        LeftBracket,
        Identifier("+".to_string()),
        Integer(1),
        Integer(2),
        Integer(3),
        Integer(4),
        Integer(5),
        RightBracket,
        Integer(15),
        RightBracket,
        LeftBracket,
        Identifier("print".to_string()),
        String("Yeah".to_string()),
        RightBracket,
        LeftBracket,
        Identifier("print".to_string()),
        String("Nope".to_string()),
        RightBracket,
        RightBracket,
    ];

    assert_eq!(
        Lexer::new(input)
            .map(|t| t.unwrap().token)
            .collect::<Vec<TokenType>>(),
        tokens
    )
}
