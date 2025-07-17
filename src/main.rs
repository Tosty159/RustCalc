use std::io::{stdin, stdout, Write};

fn main() {
    println!("RustCalc prototype.");
    println!("Press 'Ctrl+c' to exit.");
    println!("Input h/H for help.");
    println!("\n");

    loop {
        let mut expression = String::new();

        print!("RustCalc> ");
        let _ = stdout().flush();

        stdin().read_line(&mut expression).expect("Incorrect expression.");
        if let Some('\n') = expression.chars().next_back() {
            expression.pop();
        }
        if let Some('\r') = expression.chars().next_back() {
            expression.pop();
        }

        println!("Echo: {expression}");
    }
}
