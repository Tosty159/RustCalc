use crate::lexer::Token;
use crate::operator::Operator;
use std::fmt::Display;

pub enum EvalError {
    InsufficientOperands(Operator),
    OutOfPlaceInteger(i64),
}

impl Display for EvalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EvalError::InsufficientOperands(op) => write!(f, "EvaluationError: Insufficient operands for {op}."),
            EvalError::OutOfPlaceInteger(n) => write!(f, "EvaluationError: Out-of-place integer: '{n}'."),
        }
    }
}

pub type EvalResult<T> = Result<T, EvalError>;

fn eval_next(queue: &mut Vec<Token>) -> EvalResult<i64> {
    let next = queue.pop().unwrap();

    match next {
        Token::Number(n) => EvalResult::Err(EvalError::OutOfPlaceInteger(n)),
        Token::Operator(op) => {
            let num_operands = op.num_operands();
            let mut operands: Vec<i64> = Vec::new();

            while operands.len() < num_operands as usize {
                let last = queue.last().cloned();
                if last.is_none() {
                    return EvalResult::Err(EvalError::InsufficientOperands(op));
                }
                
                match last.unwrap() {
                    Token::Number(n) => {
                        queue.pop();
                        operands.push(n)
                    },
                    Token::Operator(_) => {
                        match eval_next(queue) {
                            EvalResult::Ok(n) => operands.push(n),
                            EvalResult::Err(e) => return EvalResult::Err(e),
                        }
                    },
                }
            }

            operands.reverse(); // This is because the RPN write 'rhs lhs'

            let result = match op.eval(operands) {
                Some(i) => i,
                None => return EvalResult::Err(EvalError::InsufficientOperands(op)),
            };

            EvalResult::Ok(result)
        }
    }
}

fn eval_output(queue: &mut Vec<Token>) -> EvalResult<i64> {
    let last = queue.pop().unwrap();

    match last {
        Token::Number(n) => EvalResult::Ok(n.into()),
        Token::Operator(op) => {
            let num_operands = op.num_operands();
            let mut operands: Vec<i64> = Vec::new();

            while operands.len() < num_operands as usize {
                let next = queue.last().cloned();
                if next.is_none() {
                    return EvalResult::Err(EvalError::InsufficientOperands(op));
                }
                
                match next.unwrap() {
                    Token::Number(n) => {
                        queue.pop();
                        operands.push(n.into())
                    },
                    Token::Operator(_) => {
                        match eval_next(queue) {
                            EvalResult::Ok(n) => operands.push(n),
                            EvalResult::Err(e) => return EvalResult::Err(e),
                        }
                    },
                }
            }

            operands.reverse(); // This is because the RPN write 'rhs lhs'

            let result = match op.eval(operands) {
                Some(i) => i,
                None => return EvalResult::Err(EvalError::InsufficientOperands(op)),
            };

            EvalResult::Ok(result)
        }
    }
}

pub fn shunting_yard(tokens: Vec<Token>) -> EvalResult<i64> {
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

    eval_output(&mut output_queue)
}