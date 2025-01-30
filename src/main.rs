use lexer::lexer::Lexer;
use model::{token::Token, trie::Trie};
use process::process::evaluate;

mod config;
mod lexer;
mod model;
mod parser;
mod process;

fn main() {
    let start = std::time::Instant::now();

    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage example: {} \"A ^ B\"", args[0]);
        std::process::exit(1);
    }
    let expression: &str = &args[1];

    let mut trie: Trie = Trie::new();
    trie.add_words(["~", "^", "v", "->", "<->"].to_vec());

    let mut tokens: Vec<Token> = Lexer::get_tokens(&mut Lexer {
        pos: 0,
        exp: expression,
        lex: trie,
    });

    let tokens_refs: Vec<&mut Token> = tokens.iter_mut().collect();

    evaluate(tokens_refs);

    let duration = start.elapsed();
    println!("Time elapsed: {:?}", duration);
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
