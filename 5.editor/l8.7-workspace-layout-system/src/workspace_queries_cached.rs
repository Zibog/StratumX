use crate::PanelId;
use std::collections::HashMap;

use super::CachedStateQueries;

impl<'a> CachedStateQueries<'a> {
    /// Gets or builds the panel lookup cache.
    fn get_panel_lookup_cached(&mut self) -> &HashMap<PanelId, usize> {
        if self.panel_lookup_cache.is_none() {
            let mut lookup = HashMap::new();
            for (index, panel_id) in self.session_state.open_panels.iter().enumerate() {
                lookup.insert(panel_id.clone(), index);
            }
            self.panel_lookup_cache = Some(lookup);
        }
        self.panel_lookup_cache.as_ref().unwrap()
    }

    /// Checks if a panel is open (using cached lookup).
    pub fn is_panel_open(&mut self, panel_id: &PanelId) -> bool {
        self.get_panel_lookup_cached().contains_key(panel_id)
    }

    /// Batch query for multiple panel states.
    pub fn batch_query_panels(&mut self, panel_ids: &[PanelId]) -> Vec<bool> {
        let lookup = self.get_panel_lookup_cached();
        panel_ids.iter().map(|id| lookup.contains_key(id)).collect()
    }

    pub(crate) fn focused_panel_ref(&self) -> Option<&PanelId> {
        self.session_state.focused_panel.as_ref()
    }

    pub(crate) fn open_panels_slice(&self) -> &[PanelId] {
        &self.session_state.open_panels
    }
}
