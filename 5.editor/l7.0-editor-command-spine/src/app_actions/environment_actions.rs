// Environment authoring actions

use super::ActionPayload;
use stratumx_tooling_l6_1_command_envelopes::PromotedCommand;

pub fn set_time_of_day(payload: Option<ActionPayload>) -> Option<PromotedCommand> {
    payload
        .and_then(|p| p.as_float())
        .map(|time_of_day_hours| PromotedCommand::EnvironmentSetTime { time_of_day_hours })
}

pub fn set_time(payload: Option<ActionPayload>) -> Option<PromotedCommand> {
    set_time_of_day(payload)
}

pub fn set_weather_regime(payload: Option<ActionPayload>) -> Option<PromotedCommand> {
    payload
        .and_then(|p| p.as_string().map(|s| s.to_string()))
        .map(|weather_regime| PromotedCommand::EnvironmentSetWeather { weather_regime })
}

pub fn set_weather(payload: Option<ActionPayload>) -> Option<PromotedCommand> {
    set_weather_regime(payload)
}

pub fn set_cloud_coverage(payload: Option<ActionPayload>) -> Option<PromotedCommand> {
    payload
        .and_then(|p| p.as_float())
        .map(|coverage| PromotedCommand::EnvironmentSetCloudCoverage { coverage })
}

pub fn set_fog_density(payload: Option<ActionPayload>) -> Option<PromotedCommand> {
    payload
        .and_then(|p| p.as_float())
        .map(|density| PromotedCommand::EnvironmentSetFogDensity { density })
}
