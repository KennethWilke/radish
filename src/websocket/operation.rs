use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct Operation {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    pub operation: String,
    pub parameters: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OperationReply {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    pub success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>
}

impl Operation {
    pub fn new(operation: impl ToString, id: Option<i64>) -> Self {
        let operation = operation.to_string();
        let parameters = HashMap::new();
        Operation {
            id,
            operation,
            parameters,
        }
    }

    pub fn set_parameter(&mut self, key: impl ToString, value: impl ToString) {
        let key = key.to_string();
        let value = value.to_string();
        self.parameters.insert(key, value);
    }

    pub fn make_result(&self, results: Option<HashMap<String, String>>) -> OperationReply {
        OperationReply::new_result(self.id, results)
    }

    pub fn make_error(&self, error: impl ToString) -> OperationReply {
        OperationReply::new_error(self.id, error.to_string())
    }
}

impl OperationReply {
    pub fn new_result(id: Option<i64>, results: Option<HashMap<String, String>>) -> Self {
        Self {
            id,
            success: true,
            results,
            error: None
        }
    }

    pub fn new_error(id: Option<i64>, error: String) -> Self {
        Self {
            id,
            success: false,
            results: None,
            error: Some(error)
        }
    }
}
