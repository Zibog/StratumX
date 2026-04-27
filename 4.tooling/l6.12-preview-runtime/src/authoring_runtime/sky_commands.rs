// Sky Command Handlers
// TODO: Re-implement against new engine imaging/lighting API when available.
// Currently returns stub observations to maintain vertical slice end-to-end flow.

use super::session::EditorAuthoringSession;
use editor_dto_law::WeatherRegime;
use link_egress_observations::EditorAuthoringObservation;
use link_ingress_packets::SkyCommand;

pub fn handle(
    _session: &mut EditorAuthoringSession,
    cmd: SkyCommand,
) -> Result<EditorAuthoringObservation, String> {
    match cmd {
        SkyCommand::GetSkySummary => Ok(EditorAuthoringObservation::SkySummaryRead {
            time_of_day_hours: 12.0,
            day_of_year: 172,
            latitude_deg: 45.0,
            sun_elevation_deg: 60.0,
            sun_intensity: 1.0,
            cloud_coverage: 0.3,
            fog_density: 0.0,
            rain_enabled: false,
            rain_intensity_mm_per_hour: 0.0,
            wind_vector: [0.0, 0.0, 0.0],
            storm_front_count: 0,
        }),
        SkyCommand::GetSkyBundleStatus => {
            Ok(EditorAuthoringObservation::SkyBundleStatusRead {
                status: link_egress_observations::SkyBundleStatusDto {
                    manifest_loaded: false,
                    bundle_id: None,
                    stars_status: link_egress_observations::AssetStatusDto::NotRequired,
                    moon_albedo_status: link_egress_observations::AssetStatusDto::NotRequired,
                    moon_normal_status: link_egress_observations::AssetStatusDto::NotRequired,
                    sun_disk_status: link_egress_observations::AssetStatusDto::NotRequired,
                    blue_noise_status: link_egress_observations::AssetStatusDto::NotRequired,
                    noise_source_status: link_egress_observations::AssetStatusDto::NotRequired,
                    noise_source_count: 0,
                },
            })
        }
        SkyCommand::GetSkyDiagnostics => Ok(EditorAuthoringObservation::SkyDiagnosticsRead {
            storm_front_count: 0,
            rain_enabled: false,
            rain_intensity: 0.0,
            wind_magnitude: 0.0,
            fog_density: 0.0,
            cloud_coverage: 0.3,
        }),
        SkyCommand::CaptureSkyBaseline => Ok(EditorAuthoringObservation::SkyBaselineCaptured),
        SkyCommand::ResetSkyToBaseline => Ok(EditorAuthoringObservation::SkyResetToBaseline),
        SkyCommand::RecoverDefaultSky => Ok(EditorAuthoringObservation::SkyRecoveredToDefault),
        SkyCommand::SetTimeOfDay { .. }
        | SkyCommand::SetDayOfYear { .. }
        | SkyCommand::SetLatitude { .. }
        | SkyCommand::SetCloudCoverage { .. }
        | SkyCommand::SetFogDensity { .. }
        | SkyCommand::SetRain { .. }
        | SkyCommand::SetWindVector { .. }
        | SkyCommand::StepSkySimulation { .. } => {
            // TODO: Apply to engine sky state when imaging API is restored
            Ok(EditorAuthoringObservation::SkyValueUpdated)
        }
        SkyCommand::SetWeatherRegime { regime } => {
            // Apply weather regime to world state
            let _regime: WeatherRegime = regime;
            // TODO: Apply to engine weather state when material API is restored
            Ok(EditorAuthoringObservation::SkyValueUpdated)
        }
    }
}
