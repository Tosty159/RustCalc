pub fn is_operator(ch: char) -> bool {
    ['+', '-', '*', '/']
        .contains(&ch)
}

pub fn is_valid(ch: char) -> bool {
    ch.is_digit(10) || is_operator(ch) || ch == ' '
}