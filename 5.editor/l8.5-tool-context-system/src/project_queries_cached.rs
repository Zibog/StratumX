use crate::{AudioSource, MaterialProfile};

use super::CachedStateQueries;

impl<'a> CachedStateQueries<'a> {
    /// Gets or builds the material profiles cache.
    pub fn get_material_profiles_cached(&mut self) -> &Vec<&'a MaterialProfile> {
        if self.material_profiles_cache.is_none() {
            let profiles: Vec<&MaterialProfile> =
                self.material_state.material_profiles.values().collect();
            self.material_profiles_cache = Some(profiles);
        }
        self.material_profiles_cache.as_ref().unwrap()
    }

    /// Gets or builds the audio sources cache.
    pub fn get_audio_sources_cached(&mut self) -> &Vec<&'a AudioSource> {
        if self.audio_sources_cache.is_none() {
            let sources: Vec<&AudioSource> = self.audio_state.audio_sources.values().collect();
            self.audio_sources_cache = Some(sources);
        }
        self.audio_sources_cache.as_ref().unwrap()
    }

    pub(crate) fn has_project_flag(&self) -> bool {
        !self.project_state.project_identity.project_id.is_nil()
    }

    pub(crate) fn material_profile_count(&self) -> usize {
        self.material_state.material_profiles.len()
    }

    pub(crate) fn audio_source_count(&self) -> usize {
        self.audio_state.audio_sources.len()
    }
}
