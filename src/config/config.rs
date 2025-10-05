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

pub fn get_type(lexeme: &str) -> TokenType {
    match lexeme {
        "~" => TokenType::Negation,
        "^" => TokenType::Conjunction,
        "v" => TokenType::Disjunction,
        "->" => TokenType::Conditional,
        "<->" => TokenType::BiConditional,
        _ => TokenType::Proposition,
    }
}
