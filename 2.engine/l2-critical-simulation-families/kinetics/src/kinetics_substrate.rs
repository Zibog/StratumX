use engine_core::{EngineCoreError, EngineCoreResult};
use engine_material::{
    ConsequenceTier, MaterialConsequenceEvent, MaterialRegistry, MaterialStackId,
    MaterialTriggerClass,
};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct KinematicBodyId(pub u64);

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct KinematicBody {
    pub id: KinematicBodyId,
    pub position_m: [f32; 3],
    pub velocity_m_s: [f32; 3],
    pub acceleration_m_s2: [f32; 3],
    pub material_stack_id: Option<MaterialStackId>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct KinematicsStepSummary {
    pub advanced_bodies: usize,
}

#[derive(Debug, Clone, Default)]
pub struct KineticsSubstrate {
    bodies: BTreeMap<KinematicBodyId, KinematicBody>,
}

impl KineticsSubstrate {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register_body(&mut self, body: KinematicBody) -> EngineCoreResult<()> {
        if body.id.0 == 0 {
            return Err(EngineCoreError::InvalidDescriptor(
                "kinetic body requires non-zero identity",
            ));
        }
        if self.bodies.insert(body.id, body).is_some() {
            return Err(EngineCoreError::InvalidDescriptor(
                "kinetic body identity must be unique",
            ));
        }
        Ok(())
    }

    pub fn body(&self, body_id: KinematicBodyId) -> Option<&KinematicBody> {
        self.bodies.get(&body_id)
    }

    pub fn apply_impulse(
        &mut self,
        body_id: KinematicBodyId,
        impulse_n_s: [f32; 3],
        inverse_mass: f32,
    ) -> EngineCoreResult<()> {
        if !inverse_mass.is_finite() || inverse_mass <= 0.0 {
            return Err(EngineCoreError::InvalidDescriptor(
                "kinetics impulse requires positive inverse mass",
            ));
        }
        let body = self
            .bodies
            .get_mut(&body_id)
            .ok_or(EngineCoreError::InvalidDescriptor(
                "kinetics impulse references unknown body",
            ))?;
        for (velocity, impulse) in body.velocity_m_s.iter_mut().zip(impulse_n_s) {
            *velocity += impulse * inverse_mass;
        }
        Ok(())
    }

    pub fn step_fixed(&mut self, dt_s: f32) -> EngineCoreResult<KinematicsStepSummary> {
        if !dt_s.is_finite() || dt_s <= 0.0 {
            return Err(EngineCoreError::InvalidDescriptor(
                "kinetics fixed step requires positive finite dt",
            ));
        }
        for body in self.bodies.values_mut() {
            for axis in 0..3 {
                body.velocity_m_s[axis] += body.acceleration_m_s2[axis] * dt_s;
                body.position_m[axis] += body.velocity_m_s[axis] * dt_s;
            }
        }
        Ok(KinematicsStepSummary {
            advanced_bodies: self.bodies.len(),
        })
    }

    pub fn material_response_hook(
        &self,
        materials: &MaterialRegistry,
        body_id: KinematicBodyId,
        trigger_class: MaterialTriggerClass,
    ) -> EngineCoreResult<MaterialConsequenceEvent> {
        let body = self
            .bodies
            .get(&body_id)
            .ok_or(EngineCoreError::InvalidDescriptor(
                "kinetics response hook references unknown body",
            ))?;
        let stack_id = body
            .material_stack_id
            .ok_or(EngineCoreError::InvalidDescriptor(
                "kinetic body requires material stack for response hook",
            ))?;
        materials.select_response(stack_id, trigger_class, ConsequenceTier::Dormant, &[])
    }
}
