use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Package {
    pub name: String,
    pub version: String,
    pub files: Vec<String>,
}

impl Package {
    pub fn new(name: String, version: String) -> Self {
        Self {
            name,
            version,
            files: Vec::new(),
        }
    }

    pub fn add_file(&mut self, file: String) {
        self.files.push(file);
    }
}