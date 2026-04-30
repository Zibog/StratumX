pub use damage_application::*;
pub use field_substrate::*;

mod damage_application;
mod field_substrate;

use engine_core::{EngineCoreError, EngineCoreResult, Tick};
use engine_material::MaterialRegistry;
use engine_world::{ApplySegment, WorldState};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FieldConfig {
    pub max_region_deltas: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FieldContext {
    pub tick: Tick,
    pub region_key: (i32, i32, i32),
    pub region_delta_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FieldDelta {
    pub apply_segments: Vec<ApplySegment>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FieldMetrics {
    pub region_delta_count: usize,
}

#[derive(Debug, Clone)]
pub struct FieldFamily {
    config: FieldConfig,
}

impl FieldFamily {
    pub fn new(config: FieldConfig) -> Self {
        Self { config }
    }
    pub fn simulate(
        &self,
        _world: &WorldState,
        _materials: &MaterialRegistry,
        context: FieldContext,
    ) -> EngineCoreResult<(FieldDelta, FieldMetrics)> {
        if context.region_delta_count > self.config.max_region_deltas {
            return Err(EngineCoreError::InvalidDescriptor(
                "field delta count exceeds configured ceiling",
            ));
        }
        Ok((
            FieldDelta {
                apply_segments: vec![ApplySegment {
                    region_key: context.region_key,
                    family_tags: vec![20],
                }],
            },
            FieldMetrics {
                region_delta_count: context.region_delta_count,
            },
        ))
    }
}
