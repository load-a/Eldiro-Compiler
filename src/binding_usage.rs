use crate::utilities;
use crate::environment::Environment;
use crate::value::Value;

#[derive(Debug, PartialEq)]
pub struct BindingUsage {
    name: String,
}

impl BindingUsage {
    pub fn new(source: &str) -> Result<(&str, Self), String> {
        let (source, name) = utilities::extract_identitifier(source)?;

        Ok((
            source,
            Self { name: name.to_string() }
        ))
    }

    pub(crate) fn evaluate(&self, environment: &Environment) -> Result<Value, String> {
        environment.get_binding_value(&self.name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_binding_usage() {
        assert_eq!(
            BindingUsage::new("abc"),
            Ok((
                "",
                BindingUsage {
                    name: "abc".to_string(),
                }
            ))
        );
    }

    #[test]
    fn evaluate_existing_binding() {
        let mut env = Environment::default();
        env.store_binding("abc".to_string(), Value::Number(123));

        assert_eq!(
            BindingUsage { 
                name: "abc".to_string()
            }
            .evaluate(&env),
            Ok(Value::Number(123))
        )
    }

    #[test]
    fn evaluate_nonexistant_binding() {
        let env = Environment::default();
        
        assert_eq!(
            BindingUsage { 
                name: "xyz".to_string()
            }
            .evaluate(&env),
            Err("Binding does not exist: xyz".to_string())
        )
    }
}