mod io;
mod chars;
mod lexer;
mod shunting_yard;
mod operator;

use lexer::{LexerResult, tokenize};
use shunting_yard::{EvalResult, shunting_yard};

fn main() {
    println!("RustCalc prototype.");
    println!("Press 'Ctrl+c' to exit.");
    println!("Input h/H for help.\n");

    println!("I64 min: {}", i64::MIN);
    println!("U64 max: {}", u64::MAX);

    loop {
        let expression = match io::read_input("RustCalc> ") {
            Ok(s) => s,
            Err(e) => {
                if e.kind() == std::io::ErrorKind::Interrupted { break; }

                println!("Error: {e}");
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
