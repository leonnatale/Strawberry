use std::collections::HashMap;
use crate::{error::StrawberryError, lexer::{ExpressionKind, Token, TokenKind}};

#[derive(Debug, Clone)]
enum StrawberryValue {
    String(String),
    Number(f64),
    NativeFunction(fn(Vec<StrawberryValue>) -> Result<StrawberryValue, StrawberryError>),
    Empty,
}

pub struct StrawberryParser {
    tokens: Vec<Token>,
    variables: HashMap<String, StrawberryValue>,
}

impl StrawberryParser {
    pub fn new(tokens: Vec<Token>) -> Self {
        let mut parser = Self {
            tokens,
            variables: HashMap::new(),
        };

        parser.variables.insert(
            "strawberry".into(),
            StrawberryValue::NativeFunction(|args| {
                for arg in args {
                    println!("{:?}", arg);
                }
                Ok(StrawberryValue::Empty)
            }),
        );

        parser
    }

    fn visit_expression(&mut self, token: &Token) -> Result<StrawberryValue, StrawberryError> {
        match &token.kind {
            TokenKind::Number(number) => Ok(StrawberryValue::Number(*number)),
            TokenKind::LiteralString(string) => Ok(StrawberryValue::String(string.clone())),
            TokenKind::Expression(operator, left, right) => {
                let left_value = self.parse_token(left)?;
                let right_value = self.parse_token(right)?;

                self.evaluate_expression(operator.clone(), left_value, right_value)
            }
            _ => Err(StrawberryError::semantic_error("Invalid expression token")),
        }
    }

    fn evaluate_expression(
        &self,
        operator: ExpressionKind,
        left: StrawberryValue,
        right: StrawberryValue,
    ) -> Result<StrawberryValue, StrawberryError> {
        match (left, right) {
            (StrawberryValue::Number(lhs), StrawberryValue::Number(rhs)) => {
                self.evaluate_numeric_expression(operator, lhs, rhs)
            }
            (StrawberryValue::String(lhs), StrawberryValue::String(rhs)) => {
                self.evaluate_string_expression(operator, lhs, rhs)
            }
            _ => Err(StrawberryError::semantic_error(
                "Cannot evaluate expression with mixed types",
            )),
        }
    }

    fn evaluate_numeric_expression(
        &self,
        operator: ExpressionKind,
        lhs: f64,
        rhs: f64,
    ) -> Result<StrawberryValue, StrawberryError> {
        let result = match operator {
            ExpressionKind::Add => lhs + rhs,
            ExpressionKind::Subtract => lhs - rhs,
            ExpressionKind::Multiply => lhs * rhs,
            ExpressionKind::Divide => {
                if rhs == 0.0 {
                    return Err(StrawberryError::semantic_error("Division by zero"));
                }
                lhs / rhs
            }
        };

        Ok(StrawberryValue::Number(result))
    }

    fn evaluate_string_expression(
        &self,
        operator: ExpressionKind,
        lhs: String,
        rhs: String,
    ) -> Result<StrawberryValue, StrawberryError> {
        match operator {
            ExpressionKind::Add => Ok(StrawberryValue::String(lhs + &rhs)),
            _ => Err(StrawberryError::semantic_error(
                "Invalid string operation; only concatenation is supported",
            )),
        }
    }

    fn visit_let(&mut self, token: &Token) -> Result<StrawberryValue, StrawberryError> {
        if let TokenKind::Let(name, value) = &token.kind {
            let evaluated_value = if let Some(value_token) = value {
                self.parse_token(value_token)?
            } else {
                StrawberryValue::Empty
            };

            self.variables.insert(name.clone(), evaluated_value.clone());

            Ok(evaluated_value)
        } else {
            Err(StrawberryError::semantic_error("Expected a Let token"))
        }
    }

    fn visit_identifier(&self, token: &Token) -> Result<StrawberryValue, StrawberryError> {
        if let TokenKind::Identifier(name) = &token.kind {
            if let Some(value) = self.variables.get(name) {
                Ok(value.clone())
            } else {
                Err(StrawberryError::semantic_error(&format!("Undefined variable: {}", name)))
            }
        } else {
            Err(StrawberryError::semantic_error("Expected an Identifier token"))
        }
    }

    fn visit_call(&mut self, token: &Token) -> Result<StrawberryValue, StrawberryError> {
        if let TokenKind::Call(function_name, args) = &token.kind {
            let function = self.visit_identifier(&Token {
                kind: TokenKind::Identifier(function_name.clone()),
                ..token.clone()
            })?;

            let args_values: Result<Vec<_>, _> = args
                .iter()
                .map(|arg| self.parse_token(arg))
                .collect();

            let args_values = args_values?;

            match function {
                StrawberryValue::NativeFunction(func) => func(args_values),
                _ => Err(StrawberryError::semantic_error(&format!(
                    "{} is not callable",
                    function_name
                ))),
            }
        } else {
            Err(StrawberryError::semantic_error("Expected a Call token"))
        }
    }

    fn parse_token(&mut self, token: &Token) -> Result<StrawberryValue, StrawberryError> {
        match &token.kind {
            TokenKind::LiteralString(_) | TokenKind::Number(_) | TokenKind::Expression(_, _, _) => {
                self.visit_expression(token)
            }
            TokenKind::Let(_, _) => self.visit_let(token),
            TokenKind::Identifier(_) => self.visit_identifier(token),
            TokenKind::Call(_, _) => self.visit_call(token),
            _ => Err(StrawberryError::semantic_error("Unknown token type")),
        }
    }

    pub fn run_token_stream(&mut self) {
        for token in self.tokens.clone() {
            let _result = self.parse_token(&token).unwrap();
        }
    }
}
