use super::token::Token;

#[derive(Clone)]
pub struct SyntaxNode<'a> {
    pub left: Option<Box<SyntaxNode<'a>>>,
    pub token: &'a Token,
    pub right: Option<Box<SyntaxNode<'a>>>,
}

impl<'a> SyntaxNode<'a> {
    pub fn new(
        left: Option<Box<SyntaxNode<'a>>>,
        token: &'a Token,
        right: Option<Box<SyntaxNode<'a>>>,
    ) -> Self {
        Self { left, token, right }
    }
}
