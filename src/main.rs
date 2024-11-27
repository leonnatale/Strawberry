mod error;
mod lexer;
mod parser;

use error::StrawberryErrorKind;
use lexer::StrawberryLexer;
use parser::StrawberryParser;

fn main() {
    let mut lex = StrawberryLexer::from_string(r#"
    1 + 1
    1 - 1
    1 * 1
    1 / 1
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
                _ => ()
            }
        }
    }
}