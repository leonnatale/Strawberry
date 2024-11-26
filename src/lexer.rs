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
    Call(String, Vec<Token>),
    Number(f64),
    Attribution,
    Subtract(Box<Token>, Box<Token>),
    Sum(Box<Token>, Box<Token>),
    Multiply(Box<Token>, Box<Token>),
    Divide(Box<Token>, Box<Token>),
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
    tokens: Vec<Token>,
    source: String,
    character_stream: Chars<'a>,
    current_character: Option<char>,
    index: isize,
    operators: &'a [char]
}

impl <'a> StrawberryLexer <'a> {
    pub fn from_string(source: &'a str) -> Self {
        Self {
            tokens: Vec::new(),
            source: source.to_string(),
            character_stream: source.chars(),
            current_character: Some(char::default()),
            index: -1,
            operators: &[ '=', '+', '-' ]
        }
    }

    fn next_character(&mut self) {
        self.current_character = self.character_stream.next();
        self.index += 1;
    }

    fn peek_character(&self) -> Option<char> {
        self.source.chars().nth(self.index as usize)
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
            if !current_character.is_alphanumeric() && current_character != '_' {
                break;
            }
            symbol_name.push(current_character);
            self.next_character();
        }

        let mut token_kind = TokenKind::Identifier(symbol_name.clone());

        if symbol_name == "let" {
            let mut variable_value = None;
            high_skip_whitespace!(self);
            let variable_name = self.next_token();
            treat_strawberry_error!(variable_name, syntax_error, "Set a variable name at the \"let\" statement");
            let variable_name_binding = variable_name.unwrap();
            if let TokenKind::Identifier(_) = variable_name_binding.kind {
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
    
                token_kind = TokenKind::Let(
                    Box::new(variable_name_binding),
                    variable_value
                );
            } else {
                return Err(StrawberryError::syntax_error("Let statement was expecting an identifier."));
            }
        }

        if let TokenKind::Identifier(function_name) = token_kind.clone() {
            let peek = self.peek_character();
            if let Some(peeked) = peek {
                if peeked == '(' {
                    let mut arguments = Vec::new();
                    self.next_character();
                    while let Some(current_character) = self.current_character {
                        if current_character == ')' {
                            break;
                        }

                        if current_character == ',' || current_character.is_whitespace() {
                            self.next_character();
                            continue;
                        }
                        
                        let next_token = self.next_token();

                        if let Ok(next_token_binding) = next_token {
                            arguments.push(next_token_binding);
                        } else {
                            break;
                        }
                    }

                    if self.current_character != Some(')') {
                        return Err(StrawberryError::syntax_error("Function call was not closed"));
                    }

                    self.next_character();

                    token_kind = TokenKind::Call(function_name, arguments);
                }
            }
        }

        let end = self.index as usize;
            let span = TokenSpan {
                start,
                end,
                text: self.source[start..end].to_string()
            };

        let token = Token {
            kind: token_kind,
            span
        };

        Ok(token)
    }

    fn parse_operator(&mut self) -> Result<Token, StrawberryError> {
        let mut start = self.index as usize;
        let mut operator = String::new();

        while let Some(current_character) = self.current_character {
            if !self.operators.contains(&current_character) {
                break;
            }

            operator.push(current_character);
            self.next_character();
        }

        let mut token_kind = TokenKind::Unknown;

        if operator == "=" {
            token_kind = TokenKind::Attribution;
        }

        if operator == "+" {
            let last_token = self.tokens.pop();
            high_skip_whitespace!(self);
            let next_token = self.next_token();

            if let Some(left_operand) = last_token {
                if let Ok(right_operand) = next_token {
                    start -= (left_operand.span.end - left_operand.span.start) + 1;
                    token_kind = TokenKind::Sum(
                        Box::new(left_operand),
                        Box::new(right_operand),
                    )
                }
            }
        }

        if operator == "-" {
            let unary_number = self.next_token();
            treat_strawberry_error!(unary_number, syntax_error, "Could not use the unary operator.");
            let unary_number_binding = unary_number.unwrap();
            if let TokenKind::Number(number) = unary_number_binding.kind {
                token_kind = TokenKind::Number(number * -1.0);
            } else {
                return Err(StrawberryError::syntax_error("The unary operator can be used only with numbers"));
            }
        }

        if token_kind == TokenKind::Unknown {
            return Err(StrawberryError::syntax_error(&format!("The operator \"{}\" does not exists.", operator)));
        }

        let end = self.index as usize;
        let span = TokenSpan {
            start,
            end,
            text: self.source[start..end].to_string()
        };

        let token = Token {
            kind: token_kind,
            span
        };

        Ok(token)
    }

    fn parse_number(&mut self) -> Result<Token, StrawberryError> {
        let start = self.index as usize;
        let mut number_str = String::new();
        let mut is_float = false;
    
        while let Some(current_character) = self.current_character {
            if current_character.is_digit(10) {
                number_str.push(current_character);
            } else if current_character == '.' {
                if is_float {
                    break;
                }
                is_float = true;
                number_str.push(current_character);
            } else {
                break;
            }
            self.next_character();
        }

        let number: f64 = number_str.parse().map_err(|_| {
            StrawberryError::syntax_error(&format!("\"{}\" is not a valid number", number_str))
        })?;

        let end = self.index as usize;
        let span = TokenSpan {
            start,
            end,
            text: self.source[start..end].to_string(),
        };
        let token = Token {
            kind: TokenKind::Number(number),
            span,
        };
    
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
    
            if current_character == '{' {
                return Ok(self.parse_bracket_scope()?);
            }

            if self.operators.contains(&current_character) {
                return Ok(self.parse_operator()?);
            }

            if current_character.is_digit(10) {
                return Ok(self.parse_number()?);
            }

            if current_character.is_alphabetic() {
                return Ok(self.parse_symbol()?);
            }

            return Err(StrawberryError::syntax_error(&format!("Unexpected character: \"{}\"", current_character)));
        } else {
            return Err(StrawberryError::syntax_error("Unexpected EOF."));
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