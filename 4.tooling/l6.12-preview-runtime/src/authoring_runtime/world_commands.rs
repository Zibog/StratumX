use super::session::EditorAuthoringSession;
use link_egress_observations::EditorAuthoringObservation;
use link_ingress_packets::MaterialWorldCommand;

pub fn handle(
    session: &mut EditorAuthoringSession,
    cmd: MaterialWorldCommand,
) -> Result<EditorAuthoringObservation, String> {
    session.ensure_runtime_session()?;

    match cmd {
        MaterialWorldCommand::SetBarrelWater { liters } => {
            session.proof_state.material_world.barrel_water_liters = liters.max(0.0);
            Ok(EditorAuthoringObservation::BarrelWaterSet { liters })
        }
        MaterialWorldCommand::GetBarrelWater => Ok(EditorAuthoringObservation::BarrelWaterInfo {
            liters: session.proof_state.material_world.barrel_water_liters,
        }),
        MaterialWorldCommand::SetBarrelLeak { active } => {
            session.proof_state.material_world.barrel_leak_active = active;
            Ok(EditorAuthoringObservation::BarrelLeakSet { active })
        }
        MaterialWorldCommand::IgniteFireObject => {
            let success = session.proof_state.ignite_fire_object();
            Ok(EditorAuthoringObservation::FireObjectIgnited { success })
        }
        MaterialWorldCommand::ExtinguishFireObject => {
            session.proof_state.material_world.fire_object_burning = false;
            Ok(EditorAuthoringObservation::FireObjectExtinguished)
        }
        MaterialWorldCommand::SetFireObjectWetness { wetness_percent } => {
            session.proof_state.material_world.fire_object_wetness_percent =
                wetness_percent.clamp(0.0, 100.0);
            if wetness_percent > 50.0 {
                session.proof_state.material_world.fire_object_burning = false;
            }
            Ok(EditorAuthoringObservation::FireObjectWetnessSet { wetness_percent })
        }
        MaterialWorldCommand::GetFireObjectState => {
            Ok(EditorAuthoringObservation::FireObjectState {
                burning: session.proof_state.material_world.fire_object_burning,
                wetness_percent: session.proof_state.material_world.fire_object_wetness_percent,
                fuel_remaining_percent: session
                    .proof_state
                    .material_world
                    .fire_object_fuel_remaining_percent,
            })
        }
        MaterialWorldCommand::GetSmokeParticleCount => {
            let count = session.proof_state.current_smoke_particle_count();
            Ok(EditorAuthoringObservation::SmokeParticleCount { count })
        }
        MaterialWorldCommand::UpdateMaterialWorld { delta_time } => {
            session.proof_state.update_material_world(delta_time);
            Ok(EditorAuthoringObservation::MaterialWorldUpdated { delta_time })
        }
    }
}
