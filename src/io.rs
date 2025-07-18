use crate::chars::is_valid;

use std::io::{self, stdout, Write};
use crossterm::{
    event::{self, Event, KeyCode, KeyEvent, KeyModifiers},
    terminal::{disable_raw_mode, enable_raw_mode}
};

#[derive(Debug)]
pub enum ReadError {
    Io(io::Error),
    Terminate,
    Help,
}

impl From<io::Error> for ReadError {
    fn from(err: io::Error) -> Self {
        ReadError::Io(err)
    }
}

pub type ReadResult<T> = Result<T, ReadError>;

fn get_input() -> ReadResult<String> {
    let mut input = String::new();

    let mut position: u128 = 0;
    
    // Make sure to only take valid inputs
    loop {
        if event::poll(std::time::Duration::from_millis(300))? {
            if let Event::Key(KeyEvent { code, modifiers, .. }) = event::read()? {
                match (code, modifiers) {
                    (KeyCode::Char('c'), KeyModifiers::CONTROL) => {
                        return Err(ReadError::Terminate);
                    },
                    (KeyCode::Backspace, KeyModifiers::NONE) => {
                        if position <= 0 {
                            continue;
                        }

                        write!(stdout(), "\x08 \x08")?;
                        stdout().flush()?;
                        input.pop();
                        position -= 1;
                    },
                    (KeyCode::Enter, KeyModifiers::NONE) => break,
                    (KeyCode::Char(ch), KeyModifiers::NONE) if is_valid(ch) => {
                        write!(stdout(), "{ch}")?;
                        stdout().flush()?;
                        input.push(ch);
                        position += 1;
                    },
                    (KeyCode::Char('h'), KeyModifiers::NONE) | (KeyCode::Char('H'), KeyModifiers::SHIFT) => {
                        return Err(ReadError::Help);
                    },
                    _ => {},
                }
            }
        }
    }

    Ok(input)
}

// The message is printed in the space where the user should input
// e.g. message is "Input expression: ", the user will see: "Input expression: ".
pub fn read_input(message: &str) -> ReadResult<String> {
    print!("{message}");
    let _ = stdout().flush();

    enable_raw_mode()?;

    let input = get_input();

    disable_raw_mode()?;
    println!(); // This is for aesthetic reasons

    input
}