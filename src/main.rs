mod io;
mod chars;
mod lexer;
mod shunting_yard;

use lexer::{LexerResult, tokenize};
use shunting_yard::shunting_yard;

fn main() {
    println!("RustCalc prototype.");
    println!("Press 'Ctrl+c' to exit.");
    println!("Input h/H for help.\n");

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

        let output_queue = shunting_yard(tokens);

        println!("Output: {output_queue:?}");
    }
}
