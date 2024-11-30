mod error;
mod lexer;
mod parser;

mod libs;

use std::{collections::HashMap, env::current_dir, path::PathBuf};

use error::{StrawberryError, StrawberryErrorKind};
use lexer::StrawberryLexer;
use libs::load_standard;
use parser::{StrawberryParser, StrawberryValue};

fn load_file(file_name: &str) -> Result<StrawberryValue, StrawberryError> {
    let mut file_path = PathBuf::new();
    file_path.push(current_dir().unwrap_or_default());
    file_path.push(file_name);

    if let Some(extesion) = file_path.extension() {
        if !extesion.eq("sb") {
            println!("{extesion:?} is not a valid Strawberry extension");
            return Ok(StrawberryValue::Empty);
        }
    } else {
        println!("Missing file extension");
        return Ok(StrawberryValue::Empty);
    }

    let file = std::fs::read_to_string(file_path);
    match file {
        Ok(source) => {
            let mut lexer = StrawberryLexer::from_string(&source);
            let token_stream = lexer.run_stream()?;
            let mut parser = StrawberryParser::new(
                token_stream,
                HashMap::new()
            );

            load_standard(&mut parser);

            Ok(parser.run_token_stream()?)
        },
        _ => {
            println!("Could not open the file");
            Ok(StrawberryValue::Empty)
        }
    }
}

fn main() {
    let mut arguments = std::env::args();
    let file_name = arguments.nth(1).unwrap_or_default();

    if file_name.is_empty() {
        println!("Missing file name");
        return;
    }

    match load_file(&file_name) {
        Err(error) => {
            match error.kind {
                StrawberryErrorKind::SyntaxError(message) => println!("Syntax error: {message}"),
                StrawberryErrorKind::SemanticError(message) => println!("Semantic error: {message}")
            }
        },
        _ => ()
    }
}