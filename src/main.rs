mod error;
mod lexer;

use error::StrawberryErrorKind;
use lexer::StrawberryLexer;

fn main() {
    let mut lex = StrawberryLexer::from_string("let nome");
    let result = lex.run_stream();

    match result {
        Ok(tokens) => {
            for token in tokens {
                println!("{:?}", token.kind);
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