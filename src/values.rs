use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum Value {
    Null {
        value: &'static str
    },
    Number {
        value: f64
    },
    Bool {
        value: bool
    },
    Object {
        data: HashMap<String, Value>
    }
}
