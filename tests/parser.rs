use tini::ast::{ASTType, AST};
use tini::lexer::Lexer;
use tini::parser::{ParseResult, Parser};

#[test]
fn parser_if_expression() {
    let input = r#"
    (if true
        1
        0)
    "#;
    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);

    match parser.parse_expression() {
        ParseResult::Ok(AST {
            ast:
                ASTType::If {
                    condition,
                    consequence,
                    alternative,
                },
            ..
        }) => {
            assert_eq!(condition.ast, ASTType::Bool(true));
            assert_eq!(consequence.ast, ASTType::Integer(1));
            assert_eq!(alternative.ast, ASTType::Integer(0));
        }
        e => panic!("unexpected parse result: {:?}", e),
    }

    assert!(parser.parse_expression().is_eof())
}
