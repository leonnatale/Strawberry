use std::collections::HashMap;
use crate::{error::StrawberryError, lexer::{ComparisonKind, ExpressionKind, Token, TokenKind}};

#[derive(Debug, Clone, PartialEq)]
pub enum StrawberryValue {
    String(String),
    Number(f64),
    Boolean(bool),
    NativeFunction(String, fn(Vec<StrawberryValue>, &mut StrawberryParser) -> Result<StrawberryValue, StrawberryError>),
    Function(String, Vec<String>, Vec<Box<Token>>),
    Block(Vec<Box<Token>>),
    Empty,
}

pub struct StrawberryParser {
    tokens: Vec<Token>,
    pub variables: HashMap<String, StrawberryValue>,
}

impl StrawberryParser {
    pub fn new(tokens: Vec<Token>, variables: HashMap<String, StrawberryValue>) -> Self {
        Self {
            tokens,
            variables
        }
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
    
            let args_values: Result<Vec<_>, _> = args.iter().map(|arg| self.parse_token(arg)).collect();
            let args_values = args_values?;
    
            match function {
                StrawberryValue::NativeFunction(_, func) => func(args_values, self),
    
                StrawberryValue::Function(_, params, body) => {
                    if params.len() != args_values.len() {
                        return Err(StrawberryError::semantic_error(&format!(
                            "Function {} expected {} arguments, but got {}",
                            function_name,
                            params.len(),
                            args_values.len()
                        )));
                    }
    
                    let mut scope = self.variables.clone();
                    for (param, value) in params.iter().zip(args_values.into_iter()) {
                        scope.insert(param.clone(), value);
                    }
    
                    let mut result = StrawberryValue::Empty;
                    result = StrawberryParser::new(body.iter().map(|t| *t.clone()).collect(), scope).run_token_stream()?;
                    Ok(result)
                }
    
                _ => Err(StrawberryError::semantic_error(&format!(
                    "{} is not callable",
                    function_name
                ))),
            }
        } else {
            Err(StrawberryError::semantic_error("Expected a Call token"))
        }
    }
    

    fn visit_function(&mut self, token: &Token) -> Result<StrawberryValue, StrawberryError> {
        if let TokenKind::Function(name, arguments, scope) = &token.kind {
            if let TokenKind::BracketScope(tokens) = &scope.kind {
                self.variables.insert(
                    name.clone(),
                    StrawberryValue::Function(name.clone(), arguments.clone(), tokens.clone().iter().map(|t| Box::new(t.clone())).collect()),
                );
            }
            Ok(StrawberryValue::Empty)
        } else {
            Err(StrawberryError::semantic_error("Expected a Function token"))
        }
    }

    fn evaluate_comparison(
        &self,
        operator: ComparisonKind,
        left: StrawberryValue,
        right: StrawberryValue,
    ) -> Result<StrawberryValue, StrawberryError> {
        match (operator, left, right) {
            (ComparisonKind::Equal, StrawberryValue::Number(lhs), StrawberryValue::Number(rhs)) => {
                Ok(StrawberryValue::Boolean(lhs == rhs))
            }
            (ComparisonKind::NotEqual, StrawberryValue::Number(lhs), StrawberryValue::Number(rhs)) => {
                Ok(StrawberryValue::Boolean(lhs != rhs))
            }
            (ComparisonKind::GreaterThan, StrawberryValue::Number(lhs), StrawberryValue::Number(rhs)) => {
                Ok(StrawberryValue::Boolean(lhs > rhs))
            }
            (ComparisonKind::LessThan, StrawberryValue::Number(lhs), StrawberryValue::Number(rhs)) => {
                Ok(StrawberryValue::Boolean(lhs < rhs))
            }
            (ComparisonKind::GreaterEqual, StrawberryValue::Number(lhs), StrawberryValue::Number(rhs)) => {
                Ok(StrawberryValue::Boolean(lhs >= rhs))
            }
            (ComparisonKind::LessEqual, StrawberryValue::Number(lhs), StrawberryValue::Number(rhs)) => {
                Ok(StrawberryValue::Boolean(lhs <= rhs))
            }
            (ComparisonKind::Equal, StrawberryValue::String(lhs), StrawberryValue::String(rhs)) => {
                Ok(StrawberryValue::Boolean(lhs == rhs))
            }
            (ComparisonKind::NotEqual, StrawberryValue::String(lhs), StrawberryValue::String(rhs)) => {
                Ok(StrawberryValue::Boolean(lhs != rhs))
            }
            (ComparisonKind::Equal, StrawberryValue::Boolean(lhs), StrawberryValue::Boolean(rhs)) => {
                Ok(StrawberryValue::Boolean(lhs == rhs))
            }
            (ComparisonKind::NotEqual, StrawberryValue::Boolean(lhs), StrawberryValue::Boolean(rhs)) => {
                Ok(StrawberryValue::Boolean(lhs != rhs))
            }
            _ => Err(StrawberryError::semantic_error(
                "Invalid comparison or unsupported types",
            )),
        }
    }

    fn parse_token(&mut self, token: &Token) -> Result<StrawberryValue, StrawberryError> {
        match &token.kind {
            TokenKind::Boolean(value) => Ok(StrawberryValue::Boolean(*value)),
            TokenKind::BracketScope(value) => Ok(StrawberryValue::Block(value.iter().map(|t| Box::new(t.clone())).collect())),
            TokenKind::LiteralString(_) 
            | TokenKind::Number(_) 
            | TokenKind::Expression(_, _, _) => self.visit_expression(token),
    
            TokenKind::Let(_, _) => self.visit_let(token),
    
            TokenKind::Identifier(_) => self.visit_identifier(token),
    
            TokenKind::Function(_, _, _) => self.visit_function(token),
    
            TokenKind::Call(_, _) => self.visit_call(token),
    
            TokenKind::Comparison(operator, left, right) => {
                let left_value = self.parse_token(left)?;
                let right_value = self.parse_token(right)?;
    
                self.evaluate_comparison(operator.clone(), left_value, right_value)
            }
    
            _ => Err(StrawberryError::semantic_error("Unknown token type")),
        }
    }

    pub fn run_token_stream(&mut self) -> Result<StrawberryValue, StrawberryError> {
        let mut last_result = StrawberryValue::Empty;
        for token in self.tokens.clone() {
            last_result = self.parse_token(&token)?;
        }

        Ok(last_result)
    }
}
