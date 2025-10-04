# Rust Truth Table Generator

A fast and efficient tool for generating truth tables from logical expressions, available both as a CLI and web interface.

## Features

- Supports common logical operators:
  - Negation (~)
  - Conjunction (^)
  - Disjunction (v)
  - Conditional (->)
  - Bi-conditional (<->)
- Validates expressions and catches invalid syntax
- Fast evaluation using recursive descent parsing
- Available as CLI (outputs to file) or Web UI (interactive)

**Note:** Parentheses are not currently supported. Expressions are evaluated based on operator precedence.

## Usage

### CLI

```bash
cargo run "<logical_expression>"
```

#### Examples:
```bash
cargo run "A ^ B"
cargo run --release "A ^ B" # This will run faster, it's optimized
```

This will generate a file named `result.txt` containing a truth table like:

```
|   A   |   B   |   A ^ B   |
|  true | true  |   true    |
|  true | false |   false   |
| false | true  |   false   |
| false | false |   false   |
```

### Web UI

**Try it online:** [https://spencermelo.github.io/rust-truth-table](https://spencermelo.github.io/rust-truth-table)

The tool is also available as a web interface using WebAssembly. To run locally:

1. Build the WASM module:
```bash
wasm-pack build --target web --out-dir wasm-pkg
```

2. Serve the HTML file:
```bash
python3 -m http.server 8000
```

3. Open http://localhost:8000/truth-table.html in your browser

The web interface provides an interactive UI with text input and table display, running the same Rust code compiled to WebAssembly for near-native performance.

**Note:** The GitHub Pages version is automatically built and deployed on every push to main via GitHub Actions.

## Implementation Details

- Uses a Trie data structure for efficient operator lookup
- Implements recursive descent parsing to build syntax trees
- Evaluates expressions by traversing the syntax tree
- Generates truth table variations using bitwise operations
- Outputs results to file with buffered writes for performance

## Building

```bash
cargo build
cargo build --release # The artifact on this is optimized
```

## Testing

Run test expressions to verify outputs match expected truth table values.

## Benchmarks

Performance benchmarks using `--release` mode with multithreading (tested on 2025-10-04):

| Variables | Rows Generated | Average Time |
|-----------|----------------|--------------|
| 5         | 32             | ~206.26ms    |
| 10        | 1,024          | ~2.71ms      |
| 15        | 32,768         | ~54.22ms     |
| 20        | 1,048,576      | ~2.03s       |
| 25        | 33,554,432     | ~70.30s      |

*Benchmarks run on macOS with optimized release build using parallel processing via Rayon. Each value is averaged from 5 runs. Results may vary based on hardware and CPU core count.*
