# RustCalc
A calculator made in Rust.

## How to use
Clone the repository using:
```bash
    git clone https://github.com/Tosty159/RustCalc
```

And run the code using cargo.

## Valid input
You can input a mathematical expression for RustCalc to evaluate, made up of:
- Digits(0-9).
- Basic arithmetic operators: '+', '-', '*', and '/'.

## How does it work
RustCalc enables raw mode to control what the user can input, this is to avoid checking later if the input is valid or not. Once it has received the input, it tokenizes it into numbers and operators. And using a [Shunting yard algorithm](https://en.wikipedia.org/wiki/Shunting_yard_algorithm#The_algorithm_in_detail), it evaluates the result.