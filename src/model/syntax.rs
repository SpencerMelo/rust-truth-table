use crate::config::config::TokenType;

#[derive(Clone)]
pub struct SyntaxNode {
    pub left: Option<Box<SyntaxNode>>,
    pub token_type: TokenType,
    pub value: bool,
    pub right: Option<Box<SyntaxNode>>,
}

impl SyntaxNode {
    pub fn new(
        left: Option<Box<SyntaxNode>>,
        token_type: TokenType,
        value: bool,
        right: Option<Box<SyntaxNode>>,
    ) -> Self {
        Self {
            left,
            token_type,
            value,
            right,
        }
    }
}
