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

For Test Driven Development, you can setup a Nodemon to run unit tests on every file change

```bash
$ npm i -g nodemon #if not installed, install nodemon with npm
$ nodemon --watch src -e rs --exec "cargo test --bin dayXX"
```

## Status (⭐ 12/50)

### Day 1 ⭐⭐

- Simple window looping.
- First iteration was not the most elegant one. Included for loops and manual indexing to previous items
- Watched some youtube videos and learned that there is `windows` method in rust iterator that can window iterable items. This produced very elegant functional solution.

### Day 2 ⭐⭐

- Simple command parsing and cumulating the result.
- Using Rust Enum values and pattern matching to form the results.
- Started using rust-analyzer language server in VSCode for improved inline typehints `"rust-analyzer.inlayHints.enable": true1963088820`. Seems to give nice help for understanding the type system and infered types.
- Currently the fluency in code writing is really bad. I have ansi keyboard, vscode in vim-mode and Rust as a programming language. All of these things are new to me.

### Day 3 ⭐⭐

- First part was straight forward. Basically pivoting table and then doing neccessary calculations. Most of the pain struggle was understanding how to use Rust and parse data to correct format. For example understanding the difference with String, &str and char made no sense to me at the beginning.
- Struggled to implement the second part a little. First thought that recursive algorithm would fit here nicely but ended up fighting with Rust loaning concept. Finally while loop did the trick and got the challenge solved.
- Rust is starting to be painful. Lots of problem understanding basic concepts like loaning and other stuff that is related to variable life cycle. Not completely understanding & and \* usage. Ended up in a situation with iterable where filter closure had parameter &&&str and no reason why.

### Day 4 ⭐⭐

- Implementing Bingo game.
- Lots of learning with use of structs and enums
- Lots of difficulties to get the enum value and match the input to the enums
- Second part was quite easy after doing the first part 1

### Day 5 ⭐⭐

- The trick was to parse coordinate points to a hashmaps. Hashmap where hash key is the coordinate tuple is actually quite elegant way to store grid coordinates when the size is not well known.
- Hashmap value was holding the overlapping line count.
- Part 02 diagonal calculation was causing little grey hair but made an simple solution, where I took first x-coordinate and then started to increment x and either increment or decrement y in a while loop until both x and y matched the endpoint.

### Day 6 ⭐⭐

- In part one, I used naive way to push new fishes to a vector and then counting vector length at the end.
- Vector implementation did not scale at all for part two.
- Changed to Hashmap based implementation where key was fish age and value count of the fish. In the end I can just iterate hasmap keys and accumulate fish count from the values.
- Debugged a while for a problem where rust runtime panicked. Turned out that there was overflow when calculating final result for the part02. Chagned count to u64 which worked out.
- Solution was quite elegant and really simple after changing to the Hashmap based implementation.
- Finally refactored also part01 to use Hashmap and refactored them to use common solver.