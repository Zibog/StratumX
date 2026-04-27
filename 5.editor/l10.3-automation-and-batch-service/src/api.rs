use crate::automation::*;
use crate::batch::*;
use std::collections::HashMap;

pub struct AutomationService {
    tasks: HashMap<uuid::Uuid, AutomationTask>,
    jobs: HashMap<uuid::Uuid, BatchJob>,
}

impl AutomationService {
    pub fn new() -> Self {
        Self {
            tasks: HashMap::new(),
            jobs: HashMap::new(),
        }
    }

    pub fn create_task(&mut self, name: String, script: String) -> uuid::Uuid {
        let task = AutomationTask::new(name, script);
        let id = task.id;
        self.tasks.insert(id, task);
        id
    }
}

impl Default for AutomationService {
    fn default() -> Self {
        Self::new()
    }
}