use std::io::{self, stdout, Write};
use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{enable_raw_mode, disable_raw_mode}
};

// The message is printed in the space where the user should input
// e.g. message is "Input expression: ", the user will see: "Input expression: ".
pub fn read_input(message: &str) -> io::Result<String> {
    let mut input = String::new();

    print!("{message}");
    let _ = stdout().flush();

    enable_raw_mode()?;

    // Make sure to only take valid inputs
    loop {
        if event::poll(std::time::Duration::from_millis(300))? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Enter => break,
                    KeyCode::Char(c) => {
                        if c.is_digit(10) {
                           write!(stdout(), "{c}")?;
                           stdout().flush()?;
                           input.push(c);
                        }
                    },
                    _ => {},
                }
            }
        }
    }

    disable_raw_mode()?;
    println!(); // This is for aesthetic reasons

    Ok(input)
}