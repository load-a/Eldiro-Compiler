use crate::utilities;
use crate::expression::Expression;
use crate::environment::Environment;

#[derive(Debug, PartialEq)]
pub struct BindingDefinition {
    name: String,
    value: Expression,
}

impl BindingDefinition {
    pub fn new(source: &str) -> (&str, Self) {
        let source = utilities::tag("let", source);
        let (source, _) = utilities::extract_whitespace(source);

        let (source, name) = utilities::extract_identitifier(source);
        let (source, _) = utilities::extract_whitespace(source);

        let source = utilities::tag("=", source);
        let (source, _) = utilities::extract_whitespace(source);

        let (source, value) = Expression::new(source);

        (source, Self {name: name.to_string(), value: value})
    }

    pub(crate) fn evaluate(&self, environment: &mut Environment) {
        environment.store_binding(self.name.clone(), self.value.evaluate());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::expression::{Number, Operation};

    #[test]
    fn parse_binding() {
        assert_eq!(BindingDefinition::new("let a = 10 / 2"), 
            (
                "",
                BindingDefinition {
                    name: "a".to_string(),
                    value: Expression {
                        operand: Number(10),
                        operator: Number(2),
                        operation: Operation::Div,
                    }
                }
            )
        )
    }
}