use std::fmt::{Display, Debug};

#[derive(Debug, Clone)]
pub enum OpKind {
    Unary,
    Binary,
}

#[derive(Debug, Clone, PartialEq)]
pub enum OpAssociativity {
    Left,
    Right,
}

fn precedence(op: impl AsRef<str>, is_unary: bool) -> i32 {
    let op = op.as_ref();

    if is_unary {
        return 100;
    }

    return match op {
        "*" | "/" => 2,
        "+" | "-" => 1,
        _ => -1, // Invalid operator
    }
}

fn associativity(_op: impl AsRef<str>) -> OpAssociativity {
    OpAssociativity::Left // All ops are left-associative for now
}

pub fn valid_operator(s: impl AsRef<str>) -> bool {
    let s = s.as_ref();

    ["+", "-", "*", "/"].contains(&s)
}

#[derive(Debug, Clone)]
pub struct Operator {
    kind: OpKind,
    pub precedence: i32,
    pub associativity: OpAssociativity,
    pub op: String,
}

impl Display for Operator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let kind = match self.kind {
            OpKind::Unary => "Unary",
            OpKind::Binary => "Binary",
        };

        write!(f, "Operator: '{}', kind: '{}'", self.op, kind)
    }
}

impl Operator {
    pub fn new<S: AsRef<str>>(op: S, is_unary: bool) -> Option<Self> {
        let kind = if is_unary {
            OpKind::Unary
        } else {
            OpKind::Binary
        };

        if !valid_operator(&op) {
            return None;
        }

        Some(
            Operator {
                kind,
                precedence: precedence(&op, is_unary),
                associativity: associativity(&op),
                op: String::from(op.as_ref()),
            }
        )
    }

    pub fn is_left_associative(&self) -> bool {
        self.associativity == OpAssociativity::Left
    }

    pub fn num_operands(&self) -> u32 {
        match self.kind {
            OpKind::Unary => 1,
            OpKind::Binary => 2,
        }
    }

    pub fn eval(&self, operands: Vec<i64>) -> Option<i64> {
        if operands.len() != self.num_operands() as usize {
            return None;
        }

        // The only valid OpKind is Binary
        let (lhs, rhs) = (operands.iter().nth(0).unwrap(), operands.iter().nth(1).unwrap());

        let result = match self.op.as_ref() {
            "+" => lhs + rhs,
            "-" => lhs - rhs,
            "*" => lhs * rhs,
            "/" => lhs / rhs,
            _ => 0, // Invalid operator
        };

        Some(result)
    }
}