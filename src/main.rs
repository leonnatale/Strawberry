mod error;
mod lexer;

use error::StrawberryErrorKind;
use lexer::StrawberryLexer;

fn main() {
    let mut lex = StrawberryLexer::from_string(r#"
    let hello = 'Hello'
    let world = 'world'
    let number = 1
    let float = 1.5
    let empty
    "#);
    let result = lex.run_stream();

    match result {
        Ok(tokens) => {
            for token in tokens {
                println!("Span: {:?}\nToken: {:?}\n", token.span.text, token.kind);
            }
        },
        Err(err) => {
            match err.kind {
                StrawberryErrorKind::SyntaxError(msg) => println!("Syntax error: {msg}"),
                _ => ()
            }
        }
    }
}