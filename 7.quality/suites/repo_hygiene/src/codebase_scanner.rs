use std::fs;
use std::path::Path;
use walkdir::WalkDir;

#[derive(Debug, Clone)]
pub struct CodebaseState {
    pub state_fields: Vec<StateField>,
}

#[derive(Debug, Clone)]
pub struct StateField {
    pub name: String,
    pub file_path: String,
    pub owner_type: String,
    pub field_type: String,
}

impl CodebaseState {
    pub fn new() -> Self {
        Self {
            state_fields: Vec::new(),
        }
    }

    pub fn scan_from_directory(root: &Path) -> Result<Self, String> {
        let mut state = Self::new();

        // Define the directories to scan based on the design document
        let scan_paths = vec![
            "6.apps/editor",
            "5.editor/editor-state-containers",
            "5.editor/l9.2-terrain-authoring",
            "5.editor/l9.3-material-authoring",
            "5.editor/l9.6-environment-authoring",
            "5.editor/l9.8-audio-authoring",
        ];

        for scan_path in scan_paths {
            let full_path = root.join(scan_path);
            if !full_path.exists() {
                // Skip non-existent paths (they may not be created yet)
                continue;
            }

            // Walk through the directory and find Rust files
            for entry in WalkDir::new(&full_path)
                .follow_links(false)
                .into_iter()
                .filter_map(|e| e.ok())
            {
                let path = entry.path();
                if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("rs") {
                    if let Ok(fields) = Self::scan_rust_file(path) {
                        state.state_fields.extend(fields);
                    }
                }
            }
        }

        Ok(state)
    }

    pub fn find_field(&self, name: &str) -> Option<&StateField> {
        self.state_fields.iter().find(|f| f.name == name)
    }

    pub fn fields_in_file(&self, file_path: &str) -> Vec<&StateField> {
        self.state_fields
            .iter()
            .filter(|f| f.file_path == file_path)
            .collect()
    }

    fn scan_rust_file(path: &Path) -> Result<Vec<StateField>, String> {
        let content = fs::read_to_string(path)
            .map_err(|e| format!("Failed to read file {}: {}", path.display(), e))?;

        let mut fields = Vec::new();
        let mut current_struct: Option<String> = None;
        let lines: Vec<&str> = content.lines().collect();

        for line in &lines {
            let trimmed = line.trim();

            // Detect struct definitions
            if trimmed.starts_with("pub struct ") || trimmed.starts_with("struct ") {
                if let Some(struct_name) = Self::extract_struct_name(trimmed) {
                    current_struct = Some(struct_name);
                }
            }

            // Detect end of struct
            if trimmed == "}" && current_struct.is_some() {
                current_struct = None;
            }

            // Detect field definitions within structs
            if let Some(ref owner_type) = current_struct {
                if trimmed.starts_with("pub ") && trimmed.contains(':') && !trimmed.contains("fn ")
                {
                    if let Some((field_name, field_type)) = Self::extract_field_info(trimmed) {
                        fields.push(StateField {
                            name: field_name,
                            file_path: path.display().to_string(),
                            owner_type: owner_type.clone(),
                            field_type,
                        });
                    }
                }
            }
        }

        Ok(fields)
    }

    fn extract_struct_name(line: &str) -> Option<String> {
        // Extract struct name from lines like "pub struct MyStruct {" or "struct MyStruct<T> {"
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 3 && (parts[0] == "pub" || parts[0] == "struct") {
            let name_part = if parts[0] == "pub" && parts[1] == "struct" {
                parts.get(2)?
            } else if parts[0] == "struct" {
                parts.get(1)?
            } else {
                return None;
            };

            // Remove generic parameters and opening brace
            let name = name_part.split('<').next()?.split('{').next()?.trim();

            Some(name.to_string())
        } else {
            None
        }
    }

    fn extract_field_info(line: &str) -> Option<(String, String)> {
        // Extract field name and type from lines like "pub field_name: FieldType,"
        let without_pub = line.strip_prefix("pub ")?.trim();
        let parts: Vec<&str> = without_pub.splitn(2, ':').collect();
        if parts.len() == 2 {
            let field_name = parts[0].trim().to_string();
            let field_type = parts[1].trim().trim_end_matches(',').trim().to_string();
            Some((field_name, field_type))
        } else {
            None
        }
    }
}

impl Default for CodebaseState {
    fn default() -> Self {
        Self::new()
    }
}
