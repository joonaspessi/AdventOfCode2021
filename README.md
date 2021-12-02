# Advent Of Code 2021

![build status](https://github.com/joonaspessi/AdventOfCode2021/actions/workflows/rust.yml/badge.svg)

My [Advent Of Code 2021](https://adventofcode.com/2021) solutions

Agenda is to learn [Rust](https://www.rust-lang.org/)

## Working with the code

Run a specific day solution by executing command

```bash
$ cargo run --bin dayXX
```

and run the unit tests for that specific day

```bash
$ cargo test --bin dayXX
```

## Status (⭐ 4/50)

### Day 1 ⭐⭐

- Simple window looping.
- First iteration was not the most elegant one. Included for loops and manual indexing to previous items
- Watched some youtube videos and learned that there is `windows` method in rust iterator that can window iterable items. This produced very elegant functional solution.

### Day 2 ⭐⭐

- Simple command parsing and cumulating the result.
- Using Rust Enum values and pattern matching to form the results.
- Started using rust-analyzer language server in VSCode for improved inline typehints `"rust-analyzer.inlayHints.enable": true1963088820`. Seems to give nice help for understanding the type system and infered types.
- Currently the fluency in code writing is really bad. I have ansi keyboard, vscode in vim-mode and Rust as a programming language. All of these things are new to me.