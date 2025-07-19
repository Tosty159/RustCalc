mod io;
mod chars;
mod lexer;
mod shunting_yard;
mod operator;

use io::ReadError;
use lexer::{LexerResult, tokenize};
use shunting_yard::{EvalResult, shunting_yard};

fn print_help() {
    println!("Help screen:\n");

    println!("Input a mathematical expression and RustCalc with evaluate it then print the result.");
    println!("Press 'Enter' to input the expression, and 'Ctrl+c' to stop the program.");

    println!("---------------------------------------------------------------------------------------");

    println!("Valid characters:");
    println!("- Digits(0-9).");
    println!("- Basic arithmetic operators: '+', '-', '*', and '/'.");

    println!("\nEnd of help screen.");
}

fn main() {
    println!("RustCalc prototype.");
    println!("Press 'Ctrl+c' to exit.");
    println!("Input h/H for help.\n");

    loop {
        let expression = match io::read_input("RustCalc> ") {
            Ok(s) => s,
            Err(ReadError::Io(e)) => {
                println!("IoError: {e}");
                continue;
            },
            Err(ReadError::Terminate) => break,
            Err(ReadError::Help) => {
                print_help();
                continue;
            }
        };

        let tokens = match tokenize(expression) {
            LexerResult::Ok(t) => t,
            LexerResult::Err(e) => {
                println!("{e}");
                continue;
            }
        };

        let output_queue = match shunting_yard(tokens) {
            EvalResult::Ok(o) => o,
            EvalResult::Err(e) => {
                println!("{e}");
                continue;
            }
        };

        println!("Output: {output_queue:?}");
    }
}
