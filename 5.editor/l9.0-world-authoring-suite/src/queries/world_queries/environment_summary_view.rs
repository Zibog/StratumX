use crate::owners::world_owner::{WeatherCondition, WorldOwner};
use crate::queries::ReadModel;
use uuid::Uuid;

/// World environment summary view
///
/// Read-only summary of environment state.
#[derive(Debug, Clone)]
pub struct WorldEnvironmentSummaryView {
    pub has_environment: bool,
    pub sky_profile_id: Option<Uuid>,
    pub time_of_day: f32,
    pub weather_condition: WeatherCondition,
    pub ambient_light: [f32; 3],
}

impl ReadModel<WorldOwner, WorldEnvironmentSummaryView> for WorldEnvironmentSummaryView {
    fn build(owner: &WorldOwner) -> Self {
        if let Some(env) = owner.get_environment_state() {
            Self {
                has_environment: true,
                sky_profile_id: env.sky_profile_id,
                time_of_day: env.time_of_day,
                weather_condition: env.weather_condition,
                ambient_light: env.ambient_light,
            }
        } else {
            Self {
                has_environment: false,
                sky_profile_id: None,
                time_of_day: 12.0,
                weather_condition: WeatherCondition::Clear,
                ambient_light: [0.2, 0.2, 0.2],
            }
        }
    }
}
