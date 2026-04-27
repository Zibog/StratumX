//! Audio Registry - owns audio sources, zones, and policies.

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

use crate::model::{AudioSource, AudioZone, DuckingPolicy, ObjectHandle};

/// Audio Registry - owns audio sources, zones, and policies.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct AudioRegistry {
    pub sources: BTreeMap<ObjectHandle, AudioSource>,
    pub zones: BTreeMap<ObjectHandle, AudioZone>,
    pub policies: BTreeMap<ObjectHandle, DuckingPolicy>,
    next_id: u64,
}

impl AudioRegistry {
    pub fn new() -> Self {
        Self {
            sources: BTreeMap::new(),
            zones: BTreeMap::new(),
            policies: BTreeMap::new(),
            next_id: 1,
        }
    }

    pub fn create_source(&mut self, name: String, position: [f32; 3]) -> ObjectHandle {
        let handle = ObjectHandle(self.next_id);
        self.next_id += 1;

        let source = AudioSource {
            handle,
            name,
            position: [position[0] as i32, position[1] as i32, position[2] as i32],
            emitter_class: None,
            acoustic_profile: None,
            priority: 50,
        };

        self.sources.insert(handle, source);
        handle
    }

    pub fn assign_emitter_class(
        &mut self,
        handle: ObjectHandle,
        emitter_class: String,
    ) -> Result<(), String> {
        let source = self.sources.get_mut(&handle).ok_or("Source not found")?;
        source.emitter_class = Some(emitter_class);
        Ok(())
    }

    pub fn create_zone(&mut self, name: String, indoor: bool) -> ObjectHandle {
        let handle = ObjectHandle(self.next_id);
        self.next_id += 1;

        let zone = AudioZone {
            handle,
            name,
            reverb_profile: None,
            indoor,
        };

        self.zones.insert(handle, zone);
        handle
    }

    pub fn bind_zone_reverb_profile(
        &mut self,
        handle: ObjectHandle,
        reverb_profile: String,
    ) -> Result<(), String> {
        let zone = self.zones.get_mut(&handle).ok_or("Zone not found")?;
        zone.reverb_profile = Some(reverb_profile);
        Ok(())
    }

    pub fn create_ducking_policy(
        &mut self,
        name: String,
        priority_levels: Vec<u32>,
        duck_amount: u32,
    ) -> ObjectHandle {
        let handle = ObjectHandle(self.next_id);
        self.next_id += 1;

        let policy = DuckingPolicy {
            handle,
            name,
            priority_levels,
            duck_amount,
        };

        self.policies.insert(handle, policy);
        handle
    }

    pub fn get_source(&self, handle: ObjectHandle) -> Option<&AudioSource> {
        self.sources.get(&handle)
    }

    pub fn get_zone(&self, handle: ObjectHandle) -> Option<&AudioZone> {
        self.zones.get(&handle)
    }

    pub fn get_policy(&self, handle: ObjectHandle) -> Option<&DuckingPolicy> {
        self.policies.get(&handle)
    }
}
