use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TransactionMeta {
    pub schema_version: u16,
    pub transaction_id: Option<String>,
    pub retry_of: Option<String>,
    pub source_button_id: String,
}

impl TransactionMeta {
    pub fn new(source_button_id: impl Into<String>) -> Self {
        Self {
            schema_version: 1,
            transaction_id: None,
            retry_of: None,
            source_button_id: source_button_id.into(),
        }
    }
}
