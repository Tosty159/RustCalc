use crate::lexer::Token;

fn precedence(op: impl AsRef<str>) -> i32 {
    let op = op.as_ref();

    return match op {
        "*" | "/" => 2,
        "+" | "-" => 1,
        _ => -1, // Invalid operator
    }
}

fn is_left_associative(_op: impl AsRef<str>) -> bool {
    true // All ops are left-associative for now
}

pub fn shunting_yard(tokens: Vec<Token>) -> Vec<Token> {
    let mut output_queue: Vec<Token> = Vec::new();
    let mut operator_stack: Vec<Token> = Vec::new();

    for token in tokens {
        match token {
            Token::Number(_) => output_queue.push(token),
            Token::Operator(op) => {
                while !operator_stack.is_empty() {
                    if let Token::Operator(top) = operator_stack.last().unwrap() {
                        if precedence(top) > precedence(&op) ||
                            (precedence(top) == precedence(&op) && is_left_associative(&op))
                        {
                            output_queue.push(operator_stack.pop().unwrap());
                        } else {
                            break;
                        }
                    }
                }
                operator_stack.push(Token::Operator(op.clone()));
            }
        }
    }

    while !operator_stack.is_empty() {
        output_queue.push(operator_stack.pop().unwrap());
    }

    output_queue
}