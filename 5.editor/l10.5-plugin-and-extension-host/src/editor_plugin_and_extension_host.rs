pub use serde::{Deserialize, Serialize};
pub use serde_json;
pub use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::Path,
    sync::Arc,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct PluginAndExtensionHost {
    pub registered_plugins: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct EditorProduct {
    pub plugin_host: PluginAndExtensionHost,
}

impl EditorProduct {
    pub fn add_plugin(&mut self, plugin: impl Into<String>) {
        let plugin = plugin.into();
        if !self.plugin_host.registered_plugins.contains(&plugin) {
            self.plugin_host.registered_plugins.push(plugin);
        }
    }
}
