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
}

#[cfg(test)]
mod tests {
    use super::*;
    
    // #[test]
    // fn store_new_binding() {
    //     todo!()
    // }
}