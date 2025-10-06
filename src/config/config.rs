use std::fmt;

#[derive(PartialEq, Clone)]
pub enum TokenType {
    Negation,
    Conjunction,
    Disjunction,
    Conditional,
    BiConditional,
    Proposition,
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TokenType::Negation => write!(f, "Negation"),
            TokenType::Conjunction => write!(f, "Conjunction"),
            TokenType::Disjunction => write!(f, "Disjunction"),
            TokenType::Conditional => write!(f, "Conditional"),
            TokenType::BiConditional => write!(f, "Bi-conditional"),
            TokenType::Proposition => write!(f, "Proposition"),
        }
    }
}

pub struct OperatorConfig {
    pub not_op: String,
    pub and_op: String,
    pub or_op: String,
    pub conditional_op: String,
    pub biconditional_op: String,
}

impl OperatorConfig {
    pub fn get_type(&self, lexeme: &str) -> TokenType {
        if lexeme == self.not_op {
            TokenType::Negation
        } else if lexeme == self.and_op {
            TokenType::Conjunction
        } else if lexeme == self.or_op {
            TokenType::Disjunction
        } else if lexeme == self.conditional_op {
            TokenType::Conditional
        } else if lexeme == self.biconditional_op {
            TokenType::BiConditional
        } else {
            TokenType::Proposition
        }
    }
}
