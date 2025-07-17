mod io;
mod chars;

fn main() {
    println!("RustCalc prototype.");
    println!("Press 'Ctrl+c' to exit.");
    println!("Input h/H for help.\n");

    loop {
        let expression = match io::read_input("RustCalc> ") {
            Ok(s) => s,
            Err(e) => {
                if e.kind() == std::io::ErrorKind::Interrupted { println!();break; }

                println!("Error: {e}");
                continue;
            }
        };

        println!("Echo: {expression}");
    }
}
