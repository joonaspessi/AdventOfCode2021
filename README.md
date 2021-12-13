# Advent Of Code 2021

![build status](https://github.com/joonaspessi/AdventOfCode2021/actions/workflows/rust.yml/badge.svg)

My [Advent Of Code 2021](https://adventofcode.com/2021) solutions

- Agenda is to use and learn [Rust](https://www.rust-lang.org/)
- I'm also learning to use VIM and especially neovim. I'm currently using setup from a blog [post](https://sharksforarms.dev/posts/neovim-rust/). Precisely the configuration is [this](https://github.com/sharksforarms/neovim-rust/blob/master/neovim-init-lsp-cmp-rust-tools.vim).
- Not to make it too easy, I have switched my coding keyboard from `NORDIC ISO` to `US ANSI` layout.

## Working with the code

Run a day XX solution by executing command

```bash
$ cargo run --bin dayXX
```

and run the unit tests for day XX

```bash
$ cargo test --bin dayXX
```

For Test Driven Development, you can setup a Nodemon to run unit tests on source file change

```bash
$ npm i -g nodemon #if not installed, install nodemon with npm
$ nodemon --watch src -e rs --exec "cargo test --bin dayXX"
```

## Status (⭐ 24/50)

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

- Lazy Sunday at the cottage in wilderness. Plenty of time to solve the day.
- The trick was to parse coordinate points to a hashmaps. Hashmap where hash key is the coordinate tuple is actually quite elegant way to store grid coordinates when the size is not well known.
- Hashmap value was holding the overlapping line count.
- Part 02 diagonal calculation was causing little grey hair but made an simple solution, where I took first x-coordinate and then started to increment x and either increment or decrement y in a while loop until both x and y matched the endpoint.

### Day 6 ⭐⭐

- Independence day, lots of time to solve the day.
- In part one, I used naive way to push new fishes to a vector and then counting vector length at the end. Similar problem last year [AoC 2020/23](https://adventofcode.com/2020/day/23)
- Vector implementation did not scale at all for part two.
- Changed to Hashmap based implementation where key was "fish age" and value "count of the fish". Final fish count can be resolved by iterating hashmap and accumulating values.
- Debugged a while for a problem where rust runtime panicked. Turned out that there was overflow when calculating final result for the part02. Changed count to u64 which worked out.
- Solution was quite elegant and really simple after changing to the Hashmap based implementation.
- Finally refactored also part01 and part02 to use common Hashmap based solver.

### Day07 ⭐⭐

- Again a day with a Hashmap data structure.
- Created a hashmap which key contained "crab final position" and value "needed fuel"
- Populated hashmap keys with all crab positions just iterating through all final positions and cumulating the needed fuel
- Struggled to find an answer to part two. Decided to do unit test to calculate fuel consumption.
- Seems that part02 implementation is quite slow and it could be improved by adding memoization to fuel consumption calculation
- Improved the performance by using [arithmetic progression](https://en.wikipedia.org/wiki/Arithmetic_progression)

### Day08 ⭐⭐

- Part 1 was really simple, just counting character lengths
- Had so painful moments with rust String, &str and char mappings that run out of time. Have to continue some other day with the part two.
- Solving day08 problems on day09. Ugliest code I have written in long time.
- Ended up bruteforcing the result by trying all possible permutations. It is slow and ugly but it works.
- Lots of problems with rust and the ownership model, needs more studying

### Day09 ⭐⭐

- Part01 was simple 2-dimensional vector iteration with a little twist checking neighbour values
- RUST + VIM + ANSIKEYBOARD still feels painful
- Second part was quite simple to implement with recursive algrothm. Not sure how I should include mutable parameter in recursive functions.

### Day10 ⭐⭐

- Parenthesis matching. Simple problem if you know the algorithm. From last year it was easy to call the stack based [algorithm](https://www.geeksforgeeks.org/check-for-balanced-parentheses-in-an-expression/)
- Part02 was direct continuation for part01 where you just had to to inspect the remaining stack, reverse order and add completing parenthesis.
- little bit effort to calculate the last score.

### Day11 ⭐⭐

- Adjacent array item indexing. Quite nice way is to just loop over -1..1 in two levels to get indexes for all adjacent cells.
- Hashmap as a coordinate data structure is quite awesome. It helps quite a lot filter also items that were out of bounds
- Second part was quite easy. Just checking a situation when 100 flashes in a single stage.

### Day12 ⭐⭐

- Without any knowledge how to solve this kind of problem, it first felt quite hard.
- Had to Check (cheat) how to solve this kind of problems. There was quite elegant solution to push the paths to a deque and count all the paths that leads to the end.
- Part two was quite simple after understanding the problem correctly. So right after the first small cave is explored twice, no other cave could not be explored two times.