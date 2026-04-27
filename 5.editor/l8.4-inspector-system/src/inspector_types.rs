use crate::*;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InspectorField {
    pub key: String,
    pub value: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InspectorSystem {
    pub selected: Option<ObjectHandle>,
    pub label: Option<String>,
    pub fields: Vec<InspectorField>,
}
