use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

use crate::model::FieldId;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CaptureData {
    pub timestamp: u64,
    pub field_states: BTreeMap<FieldId, FieldState>,
    pub metadata: BTreeMap<String, String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FieldState {
    pub enabled: bool,
    pub fragment_count: usize,
    pub overlay_count: usize,
    pub simulation_active: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ComparisonResult {
    pub identical: bool,
    pub differences: Vec<Difference>,
    pub summary: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Difference {
    FieldAdded(FieldId),
    FieldRemoved(FieldId),
    FieldStateChanged {
        field_id: FieldId,
        old_state: FieldState,
        new_state: FieldState,
    },
    MetadataChanged {
        key: String,
        old_value: Option<String>,
        new_value: Option<String>,
    },
}

impl CaptureData {
    pub fn new() -> Self {
        Self {
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            field_states: BTreeMap::new(),
            metadata: BTreeMap::new(),
        }
    }

    pub fn add_field_state(&mut self, field_id: FieldId, state: FieldState) {
        self.field_states.insert(field_id, state);
    }

    pub fn add_metadata(&mut self, key: String, value: String) {
        self.metadata.insert(key, value);
    }
}

impl Default for CaptureData {
    fn default() -> Self {
        Self::new()
    }
}

impl ComparisonResult {
    pub fn new(differences: Vec<Difference>) -> Self {
        let identical = differences.is_empty();
        let summary = if identical {
            "Captures are identical".to_string()
        } else {
            format!("Found {} difference(s)", differences.len())
        };
        Self {
            identical,
            differences,
            summary,
        }
    }

    pub fn compare(capture1: &CaptureData, capture2: &CaptureData) -> Self {
        let mut differences = Vec::new();

        // Check for added/removed fields
        for field_id in capture1.field_states.keys() {
            if !capture2.field_states.contains_key(field_id) {
                differences.push(Difference::FieldRemoved(*field_id));
            }
        }

        for field_id in capture2.field_states.keys() {
            if !capture1.field_states.contains_key(field_id) {
                differences.push(Difference::FieldAdded(*field_id));
            }
        }

        // Check for changed field states
        for (field_id, state1) in &capture1.field_states {
            if let Some(state2) = capture2.field_states.get(field_id) {
                if state1 != state2 {
                    differences.push(Difference::FieldStateChanged {
                        field_id: *field_id,
                        old_state: state1.clone(),
                        new_state: state2.clone(),
                    });
                }
            }
        }

        // Check for metadata changes
        let all_keys: std::collections::BTreeSet<_> = capture1
            .metadata
            .keys()
            .chain(capture2.metadata.keys())
            .collect();

        for key in all_keys {
            let old_value = capture1.metadata.get(key).cloned();
            let new_value = capture2.metadata.get(key).cloned();
            if old_value != new_value {
                differences.push(Difference::MetadataChanged {
                    key: key.clone(),
                    old_value,
                    new_value,
                });
            }
        }

        Self::new(differences)
    }
}
