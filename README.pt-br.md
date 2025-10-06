# Gerador de Tabela-Verdade em Rust

Uma ferramenta rápida e eficiente para gerar tabelas-verdade a partir de expressões lógicas. Disponível tanto em CLI quanto em interface web.

## Funcionalidades

- Suporta operadores lógicos comuns:
  - Negação (~)
  - Conjunção (^)
  - Disjunção (v)
  - Condicional (->)
  - Bicondicional (<->)
- Valida expressões e identifica sintaxe inválida
- Avaliação rápida utilizando análise sintática descendente recursiva (_recursive descent parsing_)
- Disponível em CLI (saída para arquivo) ou UI Web (interativa)

**Aviso:** parênteses não são suportados atualmente! Expressões são avaliadas com base na precedência do operador.

## Uso

### CLI

```bash
cargo run "<expressão_lógica>"
```

#### Exemplo:
```bash
cargo run "A ^ B"
cargo run --release "A ^ B" # Isso rodará mais rápido pois é otimizado
```

Esse exemplo gerará um arquivo chamado `result.txt` contendo a seguinte tablela-verdade:

```
|   A   |   B   |   A ^ B   |
|  true | true  |   true    |
|  true | false |   false   |
| false | true  |   false   |
| false | false |   false   |
```

### UI Web

**Utilize online:** [https://spencermelo.github.io/rust-truth-table](https://spencermelo.github.io/rust-truth-table)

A ferramenta também está disponível como interface web utilizando WebAssembly. Para rodar localmente:

1. Contrua o módulo WASM:
```bash
wasm-pack build --target web --out-dir wasm-pkg
```

2. Sirva o arquivo HTML:
```bash
python3 -m http.server 8000
```

3. Abra http://localhost:8000/index.html no seu navegador

A interface web oferece uma UI interativa com campo de texto para a entrada de expressões lógicas e exibe a tabela-verdade correspondente. O mesmo código em Rust é compilado para WebAssembly, mantendo um desempenho próximo ao nativo.

**Aviso:** a versão do GitHub Pages é automaticamente construída e implementada a cada push para a main via GitHub Actions.

## Detalhes da Implementação

- Utiliza uma estrutura de dados Trie para busca eficiente de operadores
- Implementa análise sintática descendente recursiva (_recursive descent parsin_) para construir árvores de sintaxe
- Avalia expressões percorrendo a árvore de sintaxe
- Gera variações de tabela-verdade utilizando lógica binária
- A saída dos resultados para arquivo utiliza gravação em buffer para melhor desempenho

## Construção

```bash
cargo build
cargo build --release # Esse artefato é otimizado
```

## Testes

Rode expressões de teste para verificar se as saídas correspondem aos valores esperados da tabela-verdade.

## Benchmarks

Benchmarks utilizando o modo `--release` como multithreading (testado em 04-10-2025):

| Variáveis | Linhas Geradas | Tempo Médio |
|-----------|----------------|-------------|
| 5         | 32             | ~206.26ms   |
| 10        | 1,024          | ~2.71ms     |
| 15        | 32,768         | ~54.22ms    |
| 20        | 1,048,576      | ~2.03s      |
| 25        | 33,554,432     | ~70.30s     |

*Benchmarks testados no macOS com contrução release otimizada usando processamento paralelo via Rayon. Cada valor é a média de 5 execuções. Resultados podem variar dependendo do hardware e do número de núcleos da CPU.*
