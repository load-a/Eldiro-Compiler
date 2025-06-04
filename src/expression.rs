use crate::utilities;
use crate::value::Value;

#[derive(Debug, PartialEq)]
pub struct Expression {
    pub operand: Number,
    pub operator: Number,
    pub operation: Operation,
}

impl Expression {
    pub fn new(source: &str) -> (&str, Self) {
        let (source, operand) = Number::new(source);
        let (source, _) = utilities::extract_whitespace(source);
        let (source, operation) = Operation::new(source);
        let (source, _) = utilities::extract_whitespace(source);
        let (source, operator) = Number::new(source);

        (source, Self { operand, operator, operation })
    }

    pub fn evaluate(&self) -> Value {
        let Number(operand) = self.operand;
        let Number(operator) = self.operator;

        let result = match self.operation {
            Operation::Add => operand + operator,
            Operation::Sub => operand - operator,
            Operation::Mul => operand * operator,
            Operation::Div => operand / operator,
        };

        Value::Number(result)
    }
}

#[derive(Debug, PartialEq)]
pub struct Number(pub i32);

impl Number {
    pub fn new(source: &str) -> (&str, Self) {
        let (source, number) = utilities::extract_digits(source);

        (source, Self(number.parse().unwrap()))
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
    pub fn new(source: &str) -> (&str, Self) {
        let (source, operation) = utilities::extract_operation(source);

        let operation = match operation {
            "+" => Self::Add,
            "-" => Self::Sub,
            "*" => Self::Mul,
            "/" => Self::Div,
            _ => unreachable!(),
        };

        (source, operation)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_number() {
        assert_eq!(Number::new("123"), ("", Number(123)));
    }

    #[test]
    fn parse_add_operation() {
        assert_eq!(Operation::new("+"), ("", Operation::Add));
    }

    #[test]
    fn parse_subtract_operation() {
        assert_eq!(Operation::new("-"), ("", Operation::Sub));
    }

    #[test]
    fn parse_multiply_operation() {
        assert_eq!(Operation::new("*"), ("", Operation::Mul));
    }

    #[test]
    fn parse_divide_operation() {
        assert_eq!(Operation::new("/"), ("", Operation::Div));
    }

    #[test]
    fn parse_one_plus_two() {
        assert_eq!(
            Expression::new("1+2"),
            (
                "", 
                Expression {
                    operand: Number(1),
                    operator: Number(2),
                    operation: Operation::Add,
                },
            )
        );
    }

    #[test]
    fn evaluate_add() {
        assert_eq!(
            Expression {
                operand: Number(10),
                operator: Number(5),
                operation: Operation::Add,
            }
            .evaluate(),
            Value::Number(15)
        )
    }

    #[test]
    fn evaluate_subtract() {
        assert_eq!(
            Expression {
                operand: Number(10),
                operator: Number(5),
                operation: Operation::Sub,
            }
            .evaluate(),
            Value::Number(5)
        )
    }

    #[test]
    fn evaluate_multiply() {
        assert_eq!(
            Expression {
                operand: Number(10),
                operator: Number(5),
                operation: Operation::Mul,
            }
            .evaluate(),
            Value::Number(50)
        )
    }

    #[test]
    fn evaluate_divide() {
        assert_eq!(
            Expression {
                operand: Number(10),
                operator: Number(5),
                operation: Operation::Div,
            }
            .evaluate(),
            Value::Number(2)
        )
    }
}
