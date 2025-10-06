use rust_truth_table::{
    config::config::OperatorConfig,
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

    let operator_config = OperatorConfig {
        not_op: "~".to_string(),
        and_op: "^".to_string(),
        or_op: "v".to_string(),
        conditional_op: "->".to_string(),
        biconditional_op: "<->".to_string(),
    };

    let mut tokens: Vec<Token> = Lexer::get_tokens(&mut Lexer {
        pos: 0,
        exp: expression,
        lex: trie,
        config: &operator_config,
    });

    let tokens_refs: Vec<&mut Token> = tokens.iter_mut().collect();

    evaluate(tokens_refs);

    let duration = start.elapsed();
    println!("Time elapsed: {:?}", duration);
}
