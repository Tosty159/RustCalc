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

#[derive(Debug, Clone)]
pub struct Operator {
    pub kind: OpKind,
    pub precedence: i32,
    pub associativity: OpAssociativity,
    pub op: String,
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

impl Operator {
    pub fn new<S: AsRef<str>>(op: S, is_unary: bool) -> Self {
        let kind = if is_unary {
            OpKind::Unary
        } else {
            OpKind::Binary
        };

        Operator {
            kind,
            precedence: precedence(&op, is_unary),
            associativity: associativity(&op),
            op: String::from(op.as_ref()),
        }
    }

    pub fn is_left_associative(&self) -> bool {
        self.associativity == OpAssociativity::Left
    }
}