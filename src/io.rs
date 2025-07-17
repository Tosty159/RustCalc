use std::io::{self, stdin, stdout, Write};

// The message is printed in the space where the user should input
// e.g. message is "Input expression: ". The user will see: "Input expression: ".
pub fn read_input(buffer: &mut String, message: &str) -> io::Result<()> {
    print!("{message}");
    let _ = stdout().flush();

    stdin().read_line(buffer)?;
    if let Some('\n') = buffer.chars().next_back() {
        buffer.pop();
    }
    if let Some('\r') = buffer.chars().next_back() {
        buffer.pop();
    }

    Ok(())
}