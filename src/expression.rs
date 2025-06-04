use crate::utilities;
use crate::value::Value;

#[derive(Debug, PartialEq)]
pub enum Expression {
    Number(Number),
    BinaryOperation { operand: Number, operator: Number, operation: Operation }
}

impl Expression {
    pub fn new(source: &str) -> Result<(&str, Self), String> {
        Self::new_binary_operation(source).or_else(|_| Self::new_number(source))
    }

    fn new_binary_operation(source: &str) -> Result<(&str, Self), String> {
        let (source, operand) = Number::new(source)?;
        let (source, _) = utilities::extract_whitespace(source);

        let (source, operation) = Operation::new(source)?;
        let (source, _) = utilities::extract_whitespace(source);

        let (source, operator) = Number::new(source)?;

        Ok((source, Self::BinaryOperation { operand, operator, operation }))
    }

    fn new_number(source: &str) -> Result<(&str, Self), String> {
        Number::new(source).map(|(source, number)| (source, Self::Number(number)))
    }

    pub fn evaluate(&self) -> Value {
        match self {
            Self::Number(Number(n)) => Value::Number(*n),
            Self::BinaryOperation { operand, operator, operation } => {
                let Number(operand) = operand;
                let Number(operator) = operator;

                let result = match operation {
                    Operation::Add => operand + operator,
                    Operation::Sub => operand - operator,
                    Operation::Mul => operand * operator,
                    Operation::Div => operand / operator,
                };

                Value::Number(result)
            },
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Number(pub i32);

impl Number {
    pub fn new(source: &str) -> Result<(&str, Self), String> {
        let (source, number) = utilities::extract_digits(source)?;

        Ok((source, Self(number.parse().unwrap())))
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
    pub fn new(source: &str) -> Result<(&str, Self), String> {
        utilities::tag("+", source)
            .map(|source| (source, Self::Add))
            .or_else(|_| utilities::tag("-", source).map(|source| (source, Self::Sub)))
            .or_else(|_| utilities::tag("*", source).map(|source| (source, Self::Mul)))
            .or_else(|_| utilities::tag("/", source).map(|source| (source, Self::Div)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_number() {
        assert_eq!(Number::new("123"), Ok(("", Number(123))));
    }

    #[test]
    fn parse_add_operation() {
        assert_eq!(Operation::new("+"), Ok(("", Operation::Add)));
    }

    #[test]
    fn parse_subtract_operation() {
        assert_eq!(Operation::new("-"), Ok(("", Operation::Sub)));
    }

    #[test]
    fn parse_multiply_operation() {
        assert_eq!(Operation::new("*"), Ok(("", Operation::Mul)));
    }

    #[test]
    fn parse_divide_operation() {
        assert_eq!(Operation::new("/"), Ok(("", Operation::Div)));
    }

    #[test]
    fn parse_one_plus_two() {
        assert_eq!(
            Expression::new("1+2"),
            Ok((
                "", 
                Expression::BinaryOperation {
                    operand: Number(1),
                    operator: Number(2),
                    operation: Operation::Add,
                },
            ))
        );
    }

    #[test]
    fn parse_one_plus_two_with_whitespace() {
        assert_eq!(
            Expression::new("1 +   2"),
            Ok((
                "", 
                Expression::BinaryOperation {
                    operand: Number(1),
                    operator: Number(2),
                    operation: Operation::Add,
                },
            ))
        );
    }

    #[test]
    fn evaluate_add() {
        assert_eq!(
            Expression::BinaryOperation {
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
            Expression::BinaryOperation {
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
            Expression::BinaryOperation {
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
            Expression::BinaryOperation {
                operand: Number(10),
                operator: Number(5),
                operation: Operation::Div,
            }
            .evaluate(),
            Value::Number(2)
        )
    }

    #[test]
    fn parse_number_expression() {
        assert_eq!(Expression::new("456"), Ok(("", Expression::Number(Number(456)))))
    }
}
