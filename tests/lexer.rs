use tini::lexer::*;
use tini::token::*;

#[test]
fn lexer_test() {
    use crate::TokenType::*;

    let input = r#"
    (if (= (+ 1 2) 3)
        (print 1)
        (print 0))
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
        RightBracket,
        Integer(3),
        RightBracket,
        LeftBracket,
        Identifier("print".to_string()),
        Integer(1),
        RightBracket,
        LeftBracket,
        Identifier("print".to_string()),
        Integer(0),
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
