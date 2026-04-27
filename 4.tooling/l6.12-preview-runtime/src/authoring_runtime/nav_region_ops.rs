// Nav Region/Memory Command Handlers

use super::session::EditorAuthoringSession;
use link_egress_observations::EditorAuthoringObservation;
use link_ingress_packets::NavDoorInventoryCommand;

pub fn handle(
    session: &mut EditorAuthoringSession,
    cmd: NavDoorInventoryCommand,
) -> Result<EditorAuthoringObservation, String> {
    if let Some(vs) = &mut session.vertical_slice_session {
        match cmd {
            NavDoorInventoryCommand::RequestRegionLoad { region_key } => {
                vs.runtime.request_region_load(region_key)?;
                Ok(EditorAuthoringObservation::RegionLoadRequested { region_key })
            }
            NavDoorInventoryCommand::CompleteRegionLoad {
                region_key,
                size_bytes,
            } => {
                vs.runtime.complete_region_load(region_key, size_bytes)?;
                Ok(EditorAuthoringObservation::RegionLoadCompleted {
                    region_key,
                    size_bytes,
                })
            }
            NavDoorInventoryCommand::RequestRegionUnload { region_key } => {
                vs.runtime.request_region_unload(region_key)?;
                Ok(EditorAuthoringObservation::RegionUnloadRequested { region_key })
            }
            NavDoorInventoryCommand::CompleteRegionUnload { region_key } => {
                vs.runtime.complete_region_unload(region_key)?;
                Ok(EditorAuthoringObservation::RegionUnloadCompleted { region_key })
            }
            NavDoorInventoryCommand::GetMemoryPressure => {
                let pressure = vs.runtime.get_memory_pressure();
                let (current_bytes, budget_bytes) = vs.runtime.get_memory_usage();
                Ok(EditorAuthoringObservation::MemoryPressure {
                    pressure: format!("{:?}", pressure),
                    current_bytes,
                    budget_bytes,
                })
            }
            NavDoorInventoryCommand::GetRegionResidency { region_key } => {
                let residency_state = vs
                    .runtime
                    .get_region_residency(region_key)
                    .map(|state| format!("{:?}", state))
                    .unwrap_or_else(|| "Unloaded".to_string());
                Ok(EditorAuthoringObservation::RegionResidency {
                    region_key,
                    residency_state,
                })
            }
            NavDoorInventoryCommand::GetResidentRegions => {
                let regions = vs.runtime.get_resident_regions();
                let count = regions.len();
                Ok(EditorAuthoringObservation::ResidentRegions { regions, count })
            }
            NavDoorInventoryCommand::GetMemoryUsage => {
                let (current_bytes, budget_bytes) = vs.runtime.get_memory_usage();
                let usage_percent = if budget_bytes == 0 {
                    0.0
                } else {
                    current_bytes as f32 / budget_bytes as f32 * 100.0
                };
                Ok(EditorAuthoringObservation::MemoryUsage {
                    current_bytes,
                    budget_bytes,
                    usage_percent,
                })
            }
            NavDoorInventoryCommand::WorldPosToRegion { position } => {
                let region_key = vs.runtime.world_pos_to_region(position);
                Ok(EditorAuthoringObservation::RegionKeyFromPosition {
                    position,
                    region_key,
                })
            }
            _ => Err(format!(
                "Region command handler received non-region command: {:?}",
                cmd
            )),
        }
    } else {
        Err("Runtime session not initialized".into())
    }
}
