mod io;

fn main() {
    println!("RustCalc prototype.");
    println!("Press 'Ctrl+c' to exit.");
    println!("Input h/H for help.\n");

    loop {
        let mut expression = String::new();

        match io::read_input(&mut expression, "RustCalc> ") {
            Ok(()) => {},
            Err(e) => {
                println!("Error: {e}");
                continue;
            }
        }

        println!("Echo: {expression}");
    }
}
