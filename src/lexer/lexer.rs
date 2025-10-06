use crate::{
    config::config::OperatorConfig,
    model::{token::Token, trie::Trie},
};

pub struct Lexer<'a> {
    pub pos: usize,
    pub exp: &'a str,
    pub lex: Trie,
    pub config: &'a OperatorConfig,
}

impl<'a> Lexer<'a> {
    pub fn get_tokens(&mut self) -> Result<Vec<Token>, String> {
        self.pos = 0;

        let mut tokens = Vec::new();
        let mut buffer = String::new();

        for (i, letter) in self.exp.chars().enumerate() {
            self.pos = i;

            match letter {
                ' ' => {
                    if !buffer.is_empty() && buffer.chars().all(|c| c.is_alphabetic()) {
                        tokens.push(self.create_token(&buffer));
                        buffer.clear();
                    }
                }
                _ => {
                    buffer.push(letter);

                    if self.lex.has_word(&buffer) {
                        tokens.push(self.create_token(&buffer));
                        buffer.clear();
                    } else if !self.lex.has_partial(&buffer)
                        && !buffer.chars().all(|c| c.is_alphabetic())
                    {
                        return Err(format!(
                            "Invalid character '{}' at position {}",
                            letter, self.pos
                        ));
                    }
                }
            }

            if i == self.exp.len() - 1 && !buffer.is_empty() {
                if buffer.chars().all(|c| c.is_alphabetic()) {
                    tokens.push(self.create_token(&buffer));
                } else {
                    return Err(format!(
                        "Invalid character '{}' at end of expression",
                        letter
                    ));
                }
            }
        }

        Ok(tokens)
    }

    fn create_token(&self, buffer: &str) -> Token {
        Token::new(
            self.pos - (buffer.len() - 1),
            self.pos,
            self.config.get_type(buffer),
            buffer.to_string(),
            false,
        )
    }
}
