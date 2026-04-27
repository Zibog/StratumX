use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct FieldId(pub u64);

impl FieldId {
    pub fn new(raw: u64) -> Self {
        Self(raw)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FieldConfig {
    pub name: String,
    pub enabled: bool,
    pub fracture_threshold: f32,
    pub max_fragments: usize,
    pub simulation_enabled: bool,
}

impl FieldConfig {
    pub fn new(name: String) -> Self {
        Self {
            name,
            enabled: true,
            fracture_threshold: 100.0,
            max_fragments: 100,
            simulation_enabled: true,
        }
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.name.is_empty() {
            return Err("Field name cannot be empty".to_string());
        }
        if self.fracture_threshold <= 0.0 {
            return Err("Fracture threshold must be positive".to_string());
        }
        if self.max_fragments == 0 {
            return Err("Max fragments must be greater than zero".to_string());
        }
        if self.max_fragments > 10000 {
            return Err("Max fragments exceeds reasonable limit (10000)".to_string());
        }
        Ok(())
    }
}
