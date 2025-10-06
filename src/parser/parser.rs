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

    pub fn parse(&mut self) -> SyntaxNode {
        self.curr = 0;

        // Validate no consecutive propositions
        self.validate_tokens();

        self.bi_conditional()
    }

    fn validate_tokens(&self) {
        for i in 0..self.tokens.len() - 1 {
            let current = &self.tokens[i];
            let next = &self.tokens[i + 1];

            if current.token_type == TokenType::Proposition
                && next.token_type == TokenType::Proposition
            {
                panic!(
                    "Invalid expression: Consecutive propositions '{}' and '{}' at positions {} and {}. Propositions must be connected by operators.",
                    current.lexeme,
                    next.lexeme,
                    i,
                    i + 1
                );
            }
        }
    }

    fn bi_conditional(&mut self) -> SyntaxNode {
        let left = self.conditional();

        if self.is_token(TokenType::BiConditional) {
            let token = self.get_token();
            return SyntaxNode::new(
                Some(Box::new(left)),
                token.token_type.clone(),
                token.value,
                Some(Box::new(self.bi_conditional())),
            );
        }

        return left;
    }

    fn conditional(&mut self) -> SyntaxNode {
        let left = self.disjunction();

        if self.is_token(TokenType::Conditional) {
            let token = self.get_token();
            return SyntaxNode::new(
                Some(Box::new(left)),
                token.token_type.clone(),
                token.value,
                Some(Box::new(self.conditional())),
            );
        }

        return left;
    }

    fn disjunction(&mut self) -> SyntaxNode {
        let mut left = self.conjunction();

        while self.is_token(TokenType::Disjunction) {
            let token = self.get_token();
            left = SyntaxNode::new(
                Some(Box::new(left)),
                token.token_type.clone(),
                token.value,
                Some(Box::new(self.conjunction())),
            );
        }

        return left;
    }

    fn conjunction(&mut self) -> SyntaxNode {
        let mut left = self.negation();
        while self.is_token(TokenType::Conjunction) {
            let token = self.get_token();
            left = SyntaxNode::new(
                Some(Box::new(left)),
                token.token_type.clone(),
                token.value,
                Some(Box::new(self.negation())),
            )
        }
        return left;
    }

    fn negation(&mut self) -> SyntaxNode {
        if self.is_token(TokenType::Negation) {
            let token = self.get_token();
            SyntaxNode::new(
                Some(Box::new(self.negation())),
                token.token_type.clone(),
                token.value,
                None,
            )
        } else {
            let token = self.get_token();
            SyntaxNode::new(None, token.token_type.clone(), token.value, None)
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
