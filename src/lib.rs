use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

use config::config::TokenType;
use lexer::lexer::Lexer;
use model::{token::Token, trie::Trie};
use parser::parser::Parser;
use process::process::{traverse, update_tree_values};

pub mod config;
pub mod lexer;
pub mod model;
pub mod parser;
pub mod process;

#[derive(Serialize, Deserialize)]
pub struct TruthTableResult {
    pub headers: Vec<String>,
    pub rows: Vec<Vec<String>>,
}

#[wasm_bindgen]
pub fn evaluate_expression(expression: &str) -> Result<String, String> {
    let mut trie: Trie = Trie::new();
    trie.add_words(["~", "^", "v", "->", "<->"].to_vec());

    let mut tokens: Vec<Token> = Lexer::get_tokens(&mut Lexer {
        pos: 0,
        exp: expression,
        lex: trie,
    });

    let tokens_refs: Vec<&mut Token> = tokens.iter_mut().collect();

    // Extract propositions
    let proposition_tokens: Vec<String> = tokens_refs
        .iter()
        .filter(|t| t.token_type == TokenType::Proposition)
        .map(|t| t.lexeme.to_string())
        .collect();

    let variations = create_variations(proposition_tokens.len());

    // Create headers
    let mut headers: Vec<String> = proposition_tokens.clone();
    let expression_str: String = tokens_refs
        .iter()
        .map(|t| {
            if t.lexeme == "~" {
                format!("{}", t.lexeme)
            } else {
                format!("{} ", t.lexeme)
            }
        })
        .collect();
    headers.push(expression_str.trim().to_string());

    // Parse syntax tree
    let mut parser = Parser::new(&tokens_refs);
    let syntax_tree = parser.parse();

    // Generate rows
    let mut rows: Vec<Vec<String>> = Vec::new();

    for variation in variations {
        let mut tree = syntax_tree.clone();
        let mut idx = 0;
        update_tree_values(&mut tree, &variation, &mut idx);

        let result = traverse(&tree);

        let mut row: Vec<String> = variation.iter().map(|v| v.to_string()).collect();
        row.push(result.unwrap().to_string());
        rows.push(row);
    }

    let result = TruthTableResult { headers, rows };

    serde_json::to_string(&result).map_err(|e| e.to_string())
}

fn create_variations(len: usize) -> Vec<Vec<bool>> {
    let total_variations = 1_usize << len;
    let mut variations: Vec<Vec<bool>> = Vec::with_capacity(total_variations);

    for i in 0..total_variations {
        let mut variation: Vec<bool> = Vec::with_capacity(len);
        for j in (0..len).rev() {
            let bit = (i & (1 << j)) != 0;
            variation.push(bit);
        }
        variations.push(variation);
    }
    variations
}
