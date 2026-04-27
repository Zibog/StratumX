use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OwnerInventory {
    pub version: String,
    pub entries: Vec<OwnerInventoryEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OwnerInventoryEntry {
    pub entity_name: String,
    pub current_path: String,
    pub current_owner: String,
    pub target_owner: String,
    pub classification: StateClassification,
    pub can_mutate: Vec<String>,
    pub can_read: Vec<String>,
    pub publishes_changes: Option<String>,
    pub rebuilds_cache: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum StateClassification {
    Persistable,
    Transient,
    Derived,
}

impl StateClassification {
    pub fn is_persistable(&self) -> bool {
        matches!(self, Self::Persistable)
    }

    pub fn is_transient(&self) -> bool {
        matches!(self, Self::Transient)
    }

    pub fn is_derived(&self) -> bool {
        matches!(self, Self::Derived)
    }
}

impl OwnerInventory {
    pub fn new() -> Self {
        Self {
            version: "1.0".to_string(),
            entries: Vec::new(),
        }
    }

    pub fn load_from_file(path: &Path) -> Result<Self, String> {
        let content = fs::read_to_string(path)
            .map_err(|e| format!("Failed to read file {}: {}", path.display(), e))?;

        // Try JSON first
        if path.extension().and_then(|s| s.to_str()) == Some("json") {
            serde_json::from_str(&content).map_err(|e| format!("Failed to parse JSON: {}", e))
        } else {
            // For markdown format, we'll need to parse it manually
            // For now, return an error indicating markdown parsing is not yet implemented
            Err("Markdown format parsing not yet implemented. Use JSON format.".to_string())
        }
    }

    pub fn save_to_file(&self, path: &Path) -> Result<(), String> {
        let content = if path.extension().and_then(|s| s.to_str()) == Some("json") {
            serde_json::to_string_pretty(self)
                .map_err(|e| format!("Failed to serialize to JSON: {}", e))?
        } else {
            // Generate markdown format
            self.to_markdown()
        };

        fs::write(path, content)
            .map_err(|e| format!("Failed to write file {}: {}", path.display(), e))
    }

    pub fn validate_completeness(
        &self,
        codebase_state: &super::codebase_scanner::CodebaseState,
    ) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();

        // Check that every state field in codebase appears in inventory
        for field in &codebase_state.state_fields {
            if self.find_entry(&field.name).is_none() {
                errors.push(format!(
                    "State field '{}' in {} is not in Owner_Inventory",
                    field.name, field.file_path
                ));
            }
        }

        // Check that every entry has all required metadata
        for entry in &self.entries {
            if entry.entity_name.is_empty() {
                errors.push("Entry has empty entity_name".to_string());
            }
            if entry.current_path.is_empty() {
                errors.push(format!(
                    "Entry '{}' has empty current_path",
                    entry.entity_name
                ));
            }
            if entry.current_owner.is_empty() {
                errors.push(format!(
                    "Entry '{}' has empty current_owner",
                    entry.entity_name
                ));
            }
            if entry.target_owner.is_empty() {
                errors.push(format!(
                    "Entry '{}' has empty target_owner",
                    entry.entity_name
                ));
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    pub fn find_entry(&self, entity_name: &str) -> Option<&OwnerInventoryEntry> {
        self.entries.iter().find(|e| e.entity_name == entity_name)
    }

    fn to_markdown(&self) -> String {
        let mut md = String::new();
        md.push_str("# Owner Inventory\n\n");
        md.push_str(&format!("Version: {}\n\n", self.version));
        md.push_str("| Entity Name | Current Path | Current Owner | Target Owner | Classification | Can Mutate | Can Read | Publishes Changes | Rebuilds Cache |\n");
        md.push_str("|-------------|--------------|---------------|--------------|----------------|------------|----------|-------------------|----------------|\n");

        for entry in &self.entries {
            md.push_str(&format!(
                "| {} | {} | {} | {} | {:?} | {} | {} | {} | {} |\n",
                entry.entity_name,
                entry.current_path,
                entry.current_owner,
                entry.target_owner,
                entry.classification,
                entry.can_mutate.join(", "),
                entry.can_read.join(", "),
                entry.publishes_changes.as_deref().unwrap_or("N/A"),
                entry.rebuilds_cache.as_deref().unwrap_or("N/A")
            ));
        }

        md
    }
}

impl Default for OwnerInventory {
    fn default() -> Self {
        Self::new()
    }
}
