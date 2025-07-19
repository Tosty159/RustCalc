mod io;
mod chars;
mod lexer;

use lexer::{LexerResult, tokenize};

fn main() {
    println!("RustCalc prototype.");
    println!("Press 'Ctrl+c' to exit.");
    println!("Input h/H for help.\n");

    loop {
        let expression = match io::read_input("RustCalc> ") {
            Ok(s) => s,
            Err(e) => {
                if e.kind() == std::io::ErrorKind::Interrupted { println!(); break; }

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
        
        println!("Tokens: {tokens:?}");
    }
}
