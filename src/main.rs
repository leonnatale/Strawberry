mod error;
mod lexer;
mod parser;

use error::StrawberryErrorKind;
use lexer::StrawberryLexer;
use parser::StrawberryParser;

fn main() {
    let mut lex = StrawberryLexer::from_string(r#"
    let m = 1 + 1;
    let a = 1 - 1;
    let t = 1 * 1;
    let h = 1 / 1;
    let e = 1.5 - -1;
    let m = 1.6 + 1.57;
    let a = 9.3 * -42.56;
    let t = -923.123 / -90.31;
    let i = 0 / 0;
    let c = 0.5 + -0.5;
    let s = 00 - 0000.56;

    do_math(m, a, t, h, e, m, a, t, i, c, s)
    "#);
    let result = lex.run_stream();

    match result {
        Ok(tokens) => {
            let mut parser = StrawberryParser::new(tokens);
            parser.run_token_stream();
        },
        Err(err) => {
            match err.kind {
                StrawberryErrorKind::SyntaxError(msg) => println!("Syntax error: {msg}"),
                StrawberryErrorKind::SemanticError(msg) => println!("Semantic error: {msg}"),
            }
        }
    }
}