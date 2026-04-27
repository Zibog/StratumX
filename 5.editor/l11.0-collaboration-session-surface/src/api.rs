use crate::session::*;
use std::collections::HashMap;

pub struct CollaborationService {
    sessions: HashMap<uuid::Uuid, CollaborationSession>,
}

impl CollaborationService {
    pub fn new() -> Self {
        Self {
            sessions: HashMap::new(),
        }
    }

    pub fn create_session(&mut self, name: String) -> uuid::Uuid {
        let session = CollaborationSession::new(name);
        let id = session.id;
        self.sessions.insert(id, session);
        id
    }
}

impl Default for CollaborationService {
    fn default() -> Self {
        Self::new()
    }
}