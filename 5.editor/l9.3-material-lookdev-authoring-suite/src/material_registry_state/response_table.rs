use crate::MaterialProfileId;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Response table for material interactions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseTable {
    /// Profile ID this table belongs to
    pub profile_id: MaterialProfileId,

    /// Response entries
    pub responses: HashMap<String, ResponseEntry>,
}

impl ResponseTable {
    /// Creates a new empty response table
    pub fn new(profile_id: MaterialProfileId) -> Self {
        Self {
            profile_id,
            responses: HashMap::new(),
        }
    }

    /// Adds a response entry
    pub fn add_response(&mut self, interaction_type: String, entry: ResponseEntry) {
        self.responses.insert(interaction_type, entry);
    }

    /// Gets a response entry
    pub fn get_response(&self, interaction_type: &str) -> Option<&ResponseEntry> {
        self.responses.get(interaction_type)
    }
}

/// Response entry for a specific interaction type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseEntry {
    /// Interaction type (e.g., "impact", "friction", "thermal")
    pub interaction_type: String,

    /// Response magnitude
    pub magnitude: f32,

    /// Response parameters
    pub parameters: HashMap<String, f32>,
}

impl ResponseEntry {
    /// Creates a new response entry
    pub fn new(interaction_type: String, magnitude: f32) -> Self {
        Self {
            interaction_type,
            magnitude,
            parameters: HashMap::new(),
        }
    }

    /// Adds a parameter
    pub fn add_parameter(&mut self, key: String, value: f32) {
        self.parameters.insert(key, value);
    }
}
