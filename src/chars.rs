pub fn is_valid(ch: char) -> bool {
    ch.is_digit(10) || ['+', '-', '*', '/'].contains(&ch) || ch == ' '
}