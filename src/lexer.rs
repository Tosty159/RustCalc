use crate::chars::{is_valid, is_operator};
use std::fmt::Display;

#[derive(Debug)]
pub enum Token {
    Number(u64),
    Operator(String),
}

pub enum LexerError {
    IntegerOverflow,
    InvalidCharacter(char),
}

pub enum LexerResult {
    Ok(Vec<Token>),
    Err(LexerError),
}

impl Display for LexerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LexerError::IntegerOverflow => write!(f, "LexerError: Integer overflow."),
            LexerError::InvalidCharacter(ch) => write!(f, "LexerError: Invalid character: '{ch}'."),
        }
    }
}

// 5 + 3 -> Number(5) Operator('+') Number(3)
pub fn tokenize(s: impl AsRef<str>) -> LexerResult {
    let mut chars = s.as_ref().chars().peekable();

    let mut tokens: Vec<Token> = Vec::new();

    let mut current_number: u64 = 0;
    while let Some(ch) = chars.next() {
        if !is_valid(ch) {
            return LexerResult::Err(
                LexerError::InvalidCharacter(ch)
            );
        }

        if !ch.is_digit(10) && current_number > 0 {
            tokens.push(
                Token::Number(
                    current_number
                )
            );
            current_number = 0;
        }

        if ch.is_whitespace() {
            continue;
        }

        if ch.is_digit(10) {
            let digit = (ch as u8 - b'0') as u64;
            if let Some(next) = current_number.checked_mul(10).and_then(|n| n.checked_add(digit)) {
                current_number = next;
            } else {
                return LexerResult::Err(LexerError::IntegerOverflow);
            }
        } else if is_operator(ch) {
            // All operators so far are one character long
            tokens.push(
                Token::Operator(
                    String::from(ch)
                )
            );
        }
    }

    if current_number > 0 {
        tokens.push(
            Token::Number(
                current_number
            )
        );
    }

    LexerResult::Ok(tokens)
}