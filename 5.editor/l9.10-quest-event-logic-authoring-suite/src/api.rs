use crate::quest::*;
use std::collections::HashMap;

pub struct QuestAuthoringService {
    quests: HashMap<QuestId, Quest>,
}

impl QuestAuthoringService {
    pub fn new() -> Self {
        Self {
            quests: HashMap::new(),
        }
    }

    pub fn create_quest(&mut self, name: String, description: String) -> QuestId {
        let quest = Quest::new(name, description);
        let id = quest.id;
        self.quests.insert(id, quest);
        id
    }
}

impl Default for QuestAuthoringService {
    fn default() -> Self {
        Self::new()
    }
}