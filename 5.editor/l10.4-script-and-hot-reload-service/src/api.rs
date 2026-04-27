use crate::script::*;
use crate::hot_reload::*;
use std::collections::HashMap;

pub struct ScriptService {
    scripts: HashMap<uuid::Uuid, Script>,
    hot_reload: HotReloadManager,
}

impl ScriptService {
    pub fn new() -> Self {
        Self {
            scripts: HashMap::new(),
            hot_reload: HotReloadManager::new(),
        }
    }

    pub fn create_script(&mut self, name: String, language: ScriptLanguage) -> uuid::Uuid {
        let script = Script::new(name, language);
        let id = script.id;
        self.scripts.insert(id, script);
        id
    }
}

impl Default for ScriptService {
    fn default() -> Self {
        Self::new()
    }
}