mod error;
mod lexer;
mod parser;

use std::collections::HashMap;

use error::StrawberryErrorKind;
use lexer::StrawberryLexer;
use parser::StrawberryParser;

fn main() {
    let mut lex = StrawberryLexer::from_string(r#"
    function do_math(n1, n2) {
        strawberry('We do math!')
        let be = n1 * 2 + n2 / 5.5 * 94.94 / 0.5;
        strawberry('Returning:', be)
        be
    }

    strawberry(do_math(5, 9))
    strawberry(fields_forever + ', ' + fields_forever + '!')
    "#);
    let result = lex.run_stream();

    match &result {
        Ok(tokens) => {
            let mut parser = StrawberryParser::new(tokens.to_owned(), HashMap::new());
            let _result = parser.run_token_stream();
        },
        Err(err) => {
            match &err.kind {
                StrawberryErrorKind::SyntaxError(msg) => println!("Syntax error: {msg}"),
                StrawberryErrorKind::SemanticError(msg) => println!("Semantic error: {msg}"),
            }
        }
    }
}