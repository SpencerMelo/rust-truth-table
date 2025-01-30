use crate::{
    config::config::TokenType,
    create_variations,
    model::{syntax::SyntaxNode, token::Token},
    parser::parser::Parser,
};
use std::io::{BufWriter, Write};

pub fn evaluate(mut tokens: Vec<&mut Token>) {
    let proposition_tokens: Vec<usize> = tokens
        .iter()
        .enumerate()
        .filter(|(_, t)| t.token_type == TokenType::Proposition)
        .map(|(i, _)| i)
        .collect();

    let variations = create_variations(proposition_tokens.len());

    // Create table header
    let file = std::fs::OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open("result.txt")
        .unwrap();

    let mut writer = BufWriter::new(file);

    // Write header row with propositions and full expression
    let mut header = String::new();
    for token in tokens.iter() {
        if token.token_type == TokenType::Proposition {
            header.push_str(&format!("| {: ^5} ", token.lexeme));
        }
    }

    let expression_str = &format!(
        "| {: ^10}|",
        &tokens
            .iter()
            .map(|t| {
                if t.lexeme == "~" {
                    format!("{}", t.lexeme)
                } else {
                    format!("{} ", t.lexeme)
                }
            })
            .collect::<String>()
    );

    header.push_str(expression_str);
    writeln!(writer, "{}", header).unwrap();

    // Pre-allocate a String for row data
    let mut row = String::with_capacity(header.len());

    for variation in variations {
        for i in 0..variation.len() {
            tokens[proposition_tokens[i]].value = variation[i];
        }

        let mut parser = Parser::new(&tokens);
        let syntax_tree = parser.parse();
        let result = traverse(&syntax_tree.clone());

        // Clear and reuse the row String
        row.clear();

        for token in tokens.iter() {
            if token.token_type == TokenType::Proposition {
                row.push_str(&format!("| {: ^5} ", token.value));
            }
        }
        row.push_str(&format!(
            "| {: ^width$} |",
            result.unwrap(),
            width = expression_str.len() - 4
        ));
        writeln!(writer, "{}", row).unwrap();
    }

    // Ensure all data is written before closing
    writer.flush().unwrap();
}

pub fn traverse(tree: &SyntaxNode) -> Option<bool> {
    if tree.token.token_type == TokenType::Proposition {
        return Some(tree.token.value);
    }

    let left = match &tree.left {
        Some(val) => traverse(&*val),
        None => None,
    };

    let right = match &tree.right {
        Some(val) => traverse(&*val),
        None => None,
    };

    match tree.token.token_type {
        TokenType::Negation => left.map(|x| !x),
        TokenType::Conjunction => left.zip(right).map(|(l, r)| l && r),
        TokenType::Disjunction => left.zip(right).map(|(l, r)| l || r),
        TokenType::Conditional => left.zip(right).map(|(l, r)| !l || r),
        TokenType::BiConditional => left.zip(right).map(|(l, r)| l == r),
        _ => panic!("Invalid connective"),
    }
}
