use crate::lexer::Token;

pub struct StrawberryParser {
    tokens: Vec<Token>
}

impl StrawberryParser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens
        }
    }

    pub fn run_token_stream(&mut self) {
        for token in self.tokens.clone() {
            println!("Span: {:?}\nToken: {:?}\n", token.span.text, token.kind);
        }
    }
}