use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Annotation {
    pub id: uuid::Uuid,
    pub text: String,
    pub position: [f32; 3],
    pub author: String,
}

impl Annotation {
    pub fn new(text: String, author: String) -> Self {
        Self {
            id: uuid::Uuid::new_v4(),
            text,
            position: [0.0, 0.0, 0.0],
            author,
        }
    }
}