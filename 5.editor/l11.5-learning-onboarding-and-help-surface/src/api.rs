use crate::tutorial::*;
use std::collections::HashMap;

pub struct LearningService {
    tutorials: HashMap<uuid::Uuid, Tutorial>,
}

impl LearningService {
    pub fn new() -> Self {
        Self {
            tutorials: HashMap::new(),
        }
    }

    pub fn create_tutorial(&mut self, title: String) -> uuid::Uuid {
        let tutorial = Tutorial::new(title);
        let id = tutorial.id;
        self.tutorials.insert(id, tutorial);
        id
    }
}

impl Default for LearningService {
    fn default() -> Self {
        Self::new()
    }
}