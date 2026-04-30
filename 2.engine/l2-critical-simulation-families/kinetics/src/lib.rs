pub use ballistic_simulation::*;
pub use ballistic_types::*;
pub use kinetics_substrate::*;

mod ballistic_simulation;
mod ballistic_types;
mod kinetics_substrate;

use engine_core::{EngineCoreError, EngineCoreResult, Tick};
use engine_material::MaterialRegistry;
use engine_world::{ApplySegment, WorldState};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct KineticsConfig {
    pub max_contacts: usize,
    pub max_projectiles: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct KineticsContext {
    pub tick: Tick,
    pub region_key: (i32, i32, i32),
    pub contact_count: usize,
    pub projectile_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct KineticsDelta {
    pub apply_segments: Vec<ApplySegment>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct KineticsMetrics {
    pub contacts: usize,
    pub projectiles: usize,
}

#[derive(Debug, Clone)]
pub struct KineticsFamily {
    config: KineticsConfig,
}

impl KineticsFamily {
    pub fn new(config: KineticsConfig) -> Self {
        Self { config }
    }
    pub fn simulate(
        &self,
        _world: &WorldState,
        _materials: &MaterialRegistry,
        context: KineticsContext,
    ) -> EngineCoreResult<(KineticsDelta, KineticsMetrics)> {
        if context.contact_count > self.config.max_contacts {
            return Err(EngineCoreError::InvalidDescriptor(
                "contact count exceeds configured ceiling",
            ));
        }
        if context.projectile_count > self.config.max_projectiles {
            return Err(EngineCoreError::InvalidDescriptor(
                "projectile count exceeds configured ceiling",
            ));
        }
        Ok((
            KineticsDelta {
                apply_segments: vec![ApplySegment {
                    region_key: context.region_key,
                    family_tags: vec![10],
                }],
            },
            KineticsMetrics {
                contacts: context.contact_count,
                projectiles: context.projectile_count,
            },
        ))
    }
}
