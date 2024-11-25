#[derive(Debug)]
pub enum StrawberryErrorKind {
    SyntaxError(String),
    SemanticError(String)
}

#[derive(Debug)]
pub struct StrawberryError {
    pub kind: StrawberryErrorKind
}

impl StrawberryError {
    pub fn syntax_error(message: &str) -> Self {
        Self {
            kind: StrawberryErrorKind::SyntaxError(message.to_string())
        }
    }
    pub fn semantic_error(message: &str) -> Self {
        Self {
            kind: StrawberryErrorKind::SemanticError(message.to_string())
        }
    }
}