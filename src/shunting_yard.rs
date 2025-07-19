use crate::lexer::Token;
use crate::operator::Operator;

pub fn shunting_yard(tokens: Vec<Token>) -> Vec<Token> {
    let mut output_queue: Vec<Token> = Vec::new();
    let mut operator_stack: Vec<Operator> = Vec::new();

    for token in tokens {
        match token {
            Token::Number(_) => output_queue.push(token),
            Token::Operator(op1) => {
                while !operator_stack.is_empty() {
                    let op2 = operator_stack.last().unwrap();

                    if op2.precedence > op1.precedence ||
                        (op2.precedence == op1.precedence && op1.is_left_associative())
                    {
                        output_queue.push(
                            Token::Operator(
                                operator_stack.pop().unwrap()
                            )
                        );
                    } else {
                        break;
                    }
                }
                operator_stack.push(op1.clone());
            }
        }
    }

    while !operator_stack.is_empty() {
        output_queue.push(
            Token::Operator(
                operator_stack.pop().unwrap()
            )
        );
    }

    output_queue
}