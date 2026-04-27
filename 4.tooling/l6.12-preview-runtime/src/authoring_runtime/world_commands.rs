// Material World Command Handlers
// TODO: Re-implement against new engine material_world_executor (fire_ops, fluid_ops) API when available.
// Currently returns stub observations to maintain vertical slice end-to-end flow.

use super::session::EditorAuthoringSession;
use link_egress_observations::EditorAuthoringObservation;
use link_ingress_packets::MaterialWorldCommand;

pub fn handle(
    session: &mut EditorAuthoringSession,
    cmd: MaterialWorldCommand,
) -> Result<EditorAuthoringObservation, String> {
    match cmd {
        MaterialWorldCommand::SetBarrelWater { liters } => {
            // TODO: Apply to engine fluid_ops
            session.barrel_water_liters = liters;
            Ok(EditorAuthoringObservation::BarrelWaterSet { liters })
        }
        MaterialWorldCommand::GetBarrelWater => Ok(EditorAuthoringObservation::BarrelWaterInfo {
            liters: session.barrel_water_liters,
        }),
        MaterialWorldCommand::SetBarrelLeak { active } => {
            let _ = active;
            Ok(EditorAuthoringObservation::BarrelLeakSet { active })
        }
        MaterialWorldCommand::IgniteFireObject => {
            // TODO: Apply to engine fire_ops
            session.fire_object_burning = true;
            Ok(EditorAuthoringObservation::FireObjectIgnited { success: true })
        }
        MaterialWorldCommand::ExtinguishFireObject => {
            session.fire_object_burning = false;
            Ok(EditorAuthoringObservation::FireObjectExtinguished)
        }
        MaterialWorldCommand::SetFireObjectWetness { wetness_percent } => {
            if wetness_percent > 0.5 {
                session.fire_object_burning = false;
            }
            Ok(EditorAuthoringObservation::FireObjectWetnessSet { wetness_percent })
        }
        MaterialWorldCommand::GetFireObjectState => {
            Ok(EditorAuthoringObservation::FireObjectState {
                burning: session.fire_object_burning,
                wetness_percent: 0.0,
                fuel_remaining_percent: 100.0,
            })
        }
        MaterialWorldCommand::GetSmokeParticleCount => {
            let count = if session.fire_object_burning { 50 } else { 0 };
            Ok(EditorAuthoringObservation::SmokeParticleCount { count })
        }
        MaterialWorldCommand::UpdateMaterialWorld { delta_time } => {
            // TODO: Step engine material world simulation
            let _ = delta_time;
            Ok(EditorAuthoringObservation::MaterialWorldUpdated { delta_time })
        }
    }
}
