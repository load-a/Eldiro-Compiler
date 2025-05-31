mod utilities;

#[derive(Debug, PartialEq)]
pub struct Number(pub i32);

impl Number {
    pub fn new(slice: &str) -> Self {
        Self(slice.parse().unwrap())
    }
}

#[derive(Debug, PartialEq)]
pub enum Operation {
    Add,
    Sub,
    Mul,
    Div,
}

impl Operation {
    pub fn new(slice: &str) -> Self {
        match slice {
            "+" => Self::Add,
            "-" => Self::Sub,
            "*" => Self::Mul,
            "/" => Self::Div,
            _ => panic!("Bad operator"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Expression {
    pub operand: Number,
    pub operator: Number,
    pub operation: Operation,
}

impl Expression {
    pub fn new(slice: &str) -> Self {
        let operand = Number::new(slice);
        let operator = Number::new(slice);
        let operation = Operation::new(slice);

        Self { operand, operator, operation }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_number() {
        assert_eq!(Number::new("123"), Number(123));
    }

    #[test]
    fn parse_add_operation() {
        assert_eq!(Operation::new("+"), Operation::Add)
    }

    #[test]
    fn parse_subtract_operation() {
        assert_eq!(Operation::new("-"), Operation::Sub)
    }

    #[test]
    fn parse_multiply_operation() {
        assert_eq!(Operation::new("*"), Operation::Mul)
    }

    #[test]
    fn parse_divide_operation() {
        assert_eq!(Operation::new("/"), Operation::Div)
    }

    #[test]
    fn parse_one_plus_two() {
        assert_eq!(
            Expression::new("1+2"),
            Expression {
                operand: Number(1),
                operator: Number(2),
                operation: Operation::Add,
            }
        )
    }
}
