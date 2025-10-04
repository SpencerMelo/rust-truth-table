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

Performance benchmarks using `--release` mode (tested on 2025-10-04):

| Variables | Rows Generated | Average Time |
|-----------|----------------|--------------|
| 5         | 32             | ~0.01s       |
| 10        | 1,024          | ~0.01s       |
| 15        | 32,768         | ~0.13s       |
| 20        | 1,048,576      | ~5.51s       |

*Benchmarks run on macOS with optimized release build. Results may vary based on hardware.*
