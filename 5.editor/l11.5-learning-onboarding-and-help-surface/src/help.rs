use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HelpArticle {
    pub id: uuid::Uuid,
    pub title: String,
    pub content: String,
    pub category: String,
}

impl HelpArticle {
    pub fn new(title: String, category: String) -> Self {
        Self {
            id: uuid::Uuid::new_v4(),
            title,
            content: String::new(),
            category,
        }
    }
}