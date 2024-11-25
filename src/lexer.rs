use std::str::Chars;
use crate::error::StrawberryError;

macro_rules! skip_whitespace {
    ($c:expr, $obj:expr) => {
        if $c.is_whitespace() {
            $obj.next_character();
            continue;
        }
    };
}

macro_rules! high_skip_whitespace {
    ($obj:expr) => {
        while let Some(current_character) = $obj.current_character {
            if current_character.is_whitespace() {
                $obj.next_character();
                break;
            }
        }
    };
}

macro_rules! treat_strawberry_error {
    ($val:expr, $err:ident,$msg:expr) => {
        if let Err(_) = $val {
            return Err(StrawberryError::$err($msg));
        }
    };
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    LiteralString(String),
    BracketScope(Vec<Token>),
    Identifier(String),
    Let(Box<Token>, Option<Box<Token>>),
    Attribution,
    Unknown
}

#[derive(Debug, Clone, PartialEq)]
pub struct TokenSpan {
    pub start: usize,
    pub end: usize,
    pub text: String
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub span: TokenSpan
}

#[derive(Debug)]
pub struct StrawberryLexer<'a> {
    source: String,
    character_stream: Chars<'a>,
    current_character: Option<char>,
    index: isize,
    tokens: Vec<Token>,
    operators: &'a [char]
}

impl <'a> StrawberryLexer <'a> {
    pub fn from_string(source: &'a str) -> Self {
        Self {
            source: source.to_string(),
            character_stream: source.chars(),
            current_character: Some(char::default()),
            index: -1,
            tokens: Vec::new(),
            operators: &[ '=' ]
        }
    }

    fn next_character(&mut self) {
        self.current_character = self.character_stream.next();
        self.index += 1;
    }

    fn parse_multiline_string(&mut self) -> Result<Token, StrawberryError> {
        let start = self.index as usize;
        let mut string_text = String::new();

        self.next_character();

        while let Some(current_character) = self.current_character {
            if current_character == '`' {
                break;
            }
            string_text.push(current_character);
            self.next_character();
        }

        if !matches!(self.current_character, Some('`')) {
            return Err(StrawberryError::syntax_error("Missing \"`\" at the end of the string"));
        }

        self.next_character();

        let end = self.index as usize;

        let span = TokenSpan {
            start,
            end,
            text: self.source[start..end].to_string()
        };
        let token = Token {
            kind: TokenKind::LiteralString(string_text),
            span
        };

        Ok(token)
    }

    fn parse_literal_string(&mut self) -> Result<Token, StrawberryError> {
        let start = self.index as usize;
        let mut string_text = String::new();

        self.next_character();

        while let Some(current_character) = self.current_character {
            if [ '\'', '\n' ].contains(&current_character) {
                break;
            }
            string_text.push(current_character);
            self.next_character();
        }

        let message = "Missing \"'\" at the end of the string";
        if !matches!(self.current_character, Some('\'')) {
            return Err(StrawberryError::syntax_error(&message));
        }

        self.next_character();

        let end = self.index as usize;

        let span = TokenSpan {
            start,
            end,
            text: self.source[start..end].to_string()
        };
        let token = Token {
            kind: TokenKind::LiteralString(string_text),
            span
        };

        Ok(token)
    }

    fn parse_bracket_scope(&mut self) -> Result<Token, StrawberryError> {
        let start = self.index as usize;
        let mut scope_tokens = Vec::new();
        self.next_character();

        while let Some(current_character) = self.current_character {
            if [ '}' ].contains(&current_character) {
                break;
            }

            skip_whitespace!(current_character, self);
            let last_token = self.next_token()?;
            scope_tokens.push(last_token);
        }

        let message = "Scope was not closed";
        if !matches!(self.current_character, Some('}')) {
            return Err(StrawberryError::syntax_error(&message));
        }

        self.next_character();

        let end = self.index as usize;

        let span = TokenSpan {
            start,
            end,
            text: self.source[start..end].to_string()
        };

        let token = Token {
            kind: TokenKind::BracketScope(scope_tokens),
            span
        };

        Ok(token)
    }

    fn parse_symbol(&mut self) -> Result<Token, StrawberryError> {
        let start = self.index as usize;
        let mut symbol_name = String::new();

        while let Some(current_character) = self.current_character {
            if !current_character.is_alphanumeric() {
                break;
            }
            symbol_name.push(current_character);
            self.next_character();
        }

        let end = self.index as usize;
        let span = TokenSpan {
            start,
            end,
            text: self.source[start..end].to_string()
        };

        let mut token = Token {
            kind: TokenKind::Identifier(symbol_name.clone()),
            span
        };

        if symbol_name == "let" {
            let mut variable_value = None;
            high_skip_whitespace!(self);
            let variable_name = self.next_token();
            treat_strawberry_error!(variable_name, syntax_error, "Set a variable name at the \"let\" statement"); 
            high_skip_whitespace!(self);
            let operator_token_binding = self.next_token();

            if let Ok(operator_token) = operator_token_binding {
                if operator_token.kind == TokenKind::Attribution {
                    high_skip_whitespace!(self);
                    let variable_value_binding = self.next_token();
                    treat_strawberry_error!(variable_value_binding, syntax_error, "Set a variable value at the \"let\" statement"); 
                    variable_value = Some(Box::new(variable_value_binding.unwrap()));
                }
            }

            token.kind = TokenKind::Let(
                Box::new(variable_name.unwrap()),
                variable_value
            );
        }

        Ok(token)
    }

    fn parse_operator(&mut self) -> Result<Token, StrawberryError> {
        let start = self.index as usize;
        let mut operator = String::new();

        while let Some(current_character) = self.current_character {
            if !self.operators.contains(&current_character) {
                break;
            }

            operator.push(current_character);
            self.next_character();
        }

        let end = self.index as usize;
        let span = TokenSpan {
            start,
            end,
            text: self.source[start..end].to_string()
        };

        let mut token = Token {
            kind: TokenKind::Unknown,
            span
        };

        if operator == "=" {
            token.kind = TokenKind::Attribution;
        }

        if token.kind == TokenKind::Unknown {
            return Err(StrawberryError::syntax_error(&format!("The operator \"{}\" does not exists.", operator)));
        }

        Ok(token)
    }

    fn next_token(&mut self) -> Result<Token, StrawberryError> {
        if let Some(current_character) = self.current_character {
            if current_character == '\'' {
                return Ok(self.parse_literal_string()?);
            }
    
            if current_character == '`' {
                return Ok(self.parse_multiline_string()?);
            }
    
            if current_character.is_alphabetic() {
                return Ok(self.parse_symbol()?);
            }
    
            if current_character == '{' {
                return Ok(self.parse_bracket_scope()?);
            }

            if self.operators.contains(&current_character) {
                return Ok(self.parse_operator()?);
            }

            return Err(StrawberryError::syntax_error(&format!("Unexpected character: \"{}\"", current_character)));
        } else {
            return Err(StrawberryError::syntax_error(&format!("Unexpected EOF.")));
        }
    }

    pub fn run_stream(&mut self) -> Result<Vec<Token>, StrawberryError> {
        self.next_character();
        while let Some(current_character) = self.current_character {
            skip_whitespace!(current_character, self);
            let current_token = self.next_token()?;

            self.tokens.push(current_token);
        }

        Ok(self.tokens.clone())
    }
}