use crate::config::config::TokenType;

#[derive(Clone)]
#[allow(dead_code)]
pub struct Token {
    pub start: usize,
    pub end: usize,
    pub token_type: TokenType,
    pub lexeme: String,
    pub value: bool,
}

impl Token {
    pub fn new(
        start: usize,
        end: usize,
        token_type: TokenType,
        lexeme: String,
        value: bool,
    ) -> Self {
        Self {
            start,
            end,
            token_type,
            lexeme,
            value,
        }
    }
}
