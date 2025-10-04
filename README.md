# Rust Truth Table Generator

A fast and efficient command-line tool for generating truth tables from logical expressions.

## Features

- Supports common logical operators:
  - Negation (~)
  - Conjunction (^)
  - Disjunction (v)
  - Conditional (->)
  - Bi-conditional (<->)
- Validates expressions and catches invalid syntax
- Outputs formatted truth tables to a result.txt file
- Fast evaluation using recursive descent parsing

## Usage

```bash
cargo run "<logical_expression>"
```

### Examples:
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
