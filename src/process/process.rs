use crate::{
    config::config::TokenType,
    create_variations,
    model::{syntax::SyntaxNode, token::Token},
    parser::parser::Parser,
};
use rayon::prelude::*;
use std::io::{BufWriter, Write};

pub fn evaluate(tokens: Vec<&mut Token>) {
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

    let mut parser = Parser::new(&tokens);
    let syntax_tree = parser.parse();

    // Process variations in parallel and collect results
    let rows: Vec<String> = variations
        .par_iter()
        .map(|variation| {
            // Clone tree for each thread
            let mut tree = syntax_tree.clone();

            // Update tree with new proposition values
            let mut idx = 0;
            update_tree_values(&mut tree, variation, &mut idx);

            let result = traverse(&tree);

            // Build row string
            let mut row = String::with_capacity(header.len());
            for value in variation {
                row.push_str(&format!("| {: ^5} ", value));
            }
            row.push_str(&format!(
                "| {: ^width$} |",
                result.unwrap(),
                width = expression_str.len() - 4
            ));
            row
        })
        .collect();

    // Write all rows sequentially
    for row in rows {
        writeln!(writer, "{}", row).unwrap();
    }

    // Ensure all data is written before closing
    writer.flush().unwrap();
}

pub fn update_tree_values(
    tree: &mut SyntaxNode,
    proposition_values: &[bool],
    proposition_index: &mut usize,
) {
    if tree.token_type == TokenType::Proposition {
        tree.value = proposition_values[*proposition_index];
        *proposition_index += 1;
        return;
    }

    if let Some(left) = &mut tree.left {
        update_tree_values(left, proposition_values, proposition_index);
    }

    if let Some(right) = &mut tree.right {
        update_tree_values(right, proposition_values, proposition_index);
    }
}

pub fn traverse(tree: &SyntaxNode) -> Option<bool> {
    if tree.token_type == TokenType::Proposition {
        return Some(tree.value);
    }

    let left = tree.left.as_ref().and_then(|l| traverse(l));
    let right = tree.right.as_ref().and_then(|r| traverse(r));

    match tree.token_type {
        TokenType::Negation => left.map(|x| !x),
        TokenType::Conjunction => left.zip(right).map(|(l, r)| l && r),
        TokenType::Disjunction => left.zip(right).map(|(l, r)| l || r),
        TokenType::Conditional => left.zip(right).map(|(l, r)| !l || r),
        TokenType::BiConditional => left.zip(right).map(|(l, r)| l == r),
        _ => panic!("Invalid connective"),
    }
}
