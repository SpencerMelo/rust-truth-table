use crate::{
    config::config::TokenType,
    model::{syntax::SyntaxNode, token::Token},
};

pub struct Parser<'a> {
    pub curr: usize,
    pub tokens: &'a Vec<&'a mut Token>,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a Vec<&mut Token>) -> Self {
        Self { curr: 0, tokens }
    }

    pub fn parse(&mut self) -> SyntaxNode<'a> {
        self.curr = 0;
        self.bi_conditional()
    }

    fn bi_conditional(&mut self) -> SyntaxNode<'a> {
        let left = self.conditional();

        if self.is_token(TokenType::BiConditional) {
            let operator = self.get_token();
            return SyntaxNode::new(
                Some(Box::new(left)),
                operator,
                Some(Box::new(self.bi_conditional())),
            );
        }

        return left;
    }

    fn conditional(&mut self) -> SyntaxNode<'a> {
        let left = self.disjunction();

        if self.is_token(TokenType::Conditional) {
            let operator = self.get_token();
            return SyntaxNode::new(
                Some(Box::new(left)),
                operator,
                Some(Box::new(self.conditional())),
            );
        }

        return left;
    }

    fn disjunction(&mut self) -> SyntaxNode<'a> {
        let mut left = self.conjunction();

        while self.is_token(TokenType::Disjunction) {
            left = SyntaxNode::new(
                Some(Box::new(left)),
                self.get_token(),
                Some(Box::new(self.conjunction())),
            );
        }

        return left;
    }

    fn conjunction(&mut self) -> SyntaxNode<'a> {
        let mut left = self.negation();
        while self.is_token(TokenType::Conjunction) {
            left = SyntaxNode::new(
                Some(Box::new(left)),
                self.get_token(),
                Some(Box::new(self.negation())),
            )
        }
        return left;
    }

    fn negation(&mut self) -> SyntaxNode<'a> {
        if self.is_token(TokenType::Negation) {
            let negation = self.get_token();
            SyntaxNode::new(Some(Box::new(self.negation())), negation, None)
        } else {
            SyntaxNode::new(None, self.get_token(), None)
        }
    }

    fn is_token(&self, token_type: TokenType) -> bool {
        return self.curr < self.tokens.len() && self.tokens[self.curr].token_type == token_type;
    }

    fn get_token(&mut self) -> &'a Token {
        let token = &self.tokens[self.curr];
        self.curr += 1;
        token
    }
}
