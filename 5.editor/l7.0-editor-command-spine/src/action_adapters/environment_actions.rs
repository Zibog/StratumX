//! Environment actions — time/weather/clouds
//!
//! Per canon: only assemble PromotedCommand and call submit_promoted_command.

use crate::command_spine::CommandSpine;
use stratumx_tooling_l6_1_command_envelopes::PromotedCommand;

/// Environment action adapters for command spine
pub struct EnvironmentActionAdapter<'a> {
    spine: &'a mut CommandSpine,
}

impl<'a> EnvironmentActionAdapter<'a> {
    pub fn new(spine: &'a mut CommandSpine) -> Self {
        Self { spine }
    }

    pub fn request_set_time(&mut self, time_of_day_hours: f32) {
        self.spine
            .submit_promoted_command(PromotedCommand::EnvironmentSetTime {
                time_of_day_hours,
            });
    }

    pub fn request_set_weather(&mut self, weather_regime: String) {
        self.spine
            .submit_promoted_command(PromotedCommand::EnvironmentSetWeather {
                weather_regime,
            });
    }

    pub fn request_set_cloud_coverage(&mut self, coverage: f32) {
        self.spine
            .submit_promoted_command(PromotedCommand::EnvironmentSetCloudCoverage { coverage });
    }
}
