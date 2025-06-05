use std::collections::HashMap;
use crate::value::Value;

#[derive(Debug, PartialEq, Default)]
pub(crate) struct Environment {
    bindings: HashMap<String, Value>,
}

impl Environment {
    pub(crate) fn store_binding(&mut self, name: String, value: Value) {
        self.bindings.insert(name, value);
    }

    pub(crate) fn get_binding_value(&self, name: &str) -> Result<Value, String> {
        self.bindings
        .get(name)
        .cloned()
        .ok_or_else(|| format!("Binding does not exist: {}", name))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    // #[test]
    // fn store_new_binding() {
    //     todo!()
    // }
}