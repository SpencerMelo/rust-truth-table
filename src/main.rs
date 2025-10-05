use rust_truth_table::{
    lexer::lexer::Lexer,
    model::{token::Token, trie::Trie},
    process::process::evaluate,
};

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
