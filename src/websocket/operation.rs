use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct Operation {
    pub id: Option<i64>,
    pub operation: String,
    pub parameters: HashMap<String, String>,
}

impl Operation {
    pub fn new(operation: impl AsRef<str>, id: Option<i64>) -> Self {
        let operation = operation.as_ref().to_string();
        let parameters = HashMap::new();
        Operation {
            id,
            operation,
            parameters,
        }
    }

    pub fn set_parameter(&mut self, key: impl AsRef<str>, value: impl AsRef<str>) {
        let key = key.as_ref().to_string();
        let value = value.as_ref().to_string();
        self.parameters.insert(key, value);
    }

    pub fn make_result(&self) -> Self {
        let op_name = format!("{}:result", self.operation);
        Self::new(op_name, self.id)
    }

    pub fn make_error(&self, error: impl ToString) -> Self {
        let op_name = format!("{}:error", self.operation);
        let mut operation = Self::new(op_name, self.id);
        operation.set_parameter("error", error.to_string());
        operation
    }
}
