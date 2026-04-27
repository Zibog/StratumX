// Weather scenario presets

use editor_dto_law::WeatherRegime;

pub struct WeatherScenario {
    pub name: &'static str,
    pub regime: WeatherRegime,
    pub storm_bias: f32,
    pub fog_bias: f32,
    pub rain_bias: f32,
    pub cell_count: usize,
    pub cell_intensity: f32,
}

impl WeatherScenario {
    pub fn clear_day() -> Self {
        Self {
            name: "Clear Day",
            regime: WeatherRegime::Clear,
            storm_bias: 0.0,
            fog_bias: 0.0,
            rain_bias: 0.0,
            cell_count: 0,
            cell_intensity: 0.0,
        }
    }

    pub fn scattered_clouds() -> Self {
        Self {
            name: "Scattered Clouds",
            regime: WeatherRegime::Scattered,
            storm_bias: 0.2,
            fog_bias: 0.1,
            rain_bias: 0.1,
            cell_count: 3,
            cell_intensity: 0.3,
        }
    }

    pub fn overcast() -> Self {
        Self {
            name: "Overcast",
            regime: WeatherRegime::Overcast,
            storm_bias: 0.4,
            fog_bias: 0.3,
            rain_bias: 0.3,
            cell_count: 5,
            cell_intensity: 0.5,
        }
    }

    pub fn incoming_storm() -> Self {
        Self {
            name: "Incoming Storm",
            regime: WeatherRegime::IncomingStorm,
            storm_bias: 0.7,
            fog_bias: 0.2,
            rain_bias: 0.5,
            cell_count: 4,
            cell_intensity: 0.6,
        }
    }

    pub fn heavy_storm() -> Self {
        Self {
            name: "Heavy Storm",
            regime: WeatherRegime::HeavyStorm,
            storm_bias: 1.0,
            fog_bias: 0.4,
            rain_bias: 0.9,
            cell_count: 6,
            cell_intensity: 0.9,
        }
    }

    pub fn post_storm_calm() -> Self {
        Self {
            name: "Post-Storm Calm",
            regime: WeatherRegime::PostStormCalm,
            storm_bias: 0.1,
            fog_bias: 0.5,
            rain_bias: 0.2,
            cell_count: 2,
            cell_intensity: 0.2,
        }
    }

    pub fn fog_morning() -> Self {
        Self {
            name: "Foggy Morning",
            regime: WeatherRegime::FogMorning,
            storm_bias: 0.1,
            fog_bias: 0.9,
            rain_bias: 0.1,
            cell_count: 1,
            cell_intensity: 0.2,
        }
    }

    pub fn windy_overcast() -> Self {
        Self {
            name: "Windy Overcast",
            regime: WeatherRegime::WindyOvercast,
            storm_bias: 0.5,
            fog_bias: 0.2,
            rain_bias: 0.4,
            cell_count: 7,
            cell_intensity: 0.4,
        }
    }

    pub fn all_scenarios() -> Vec<Self> {
        vec![
            Self::clear_day(),
            Self::scattered_clouds(),
            Self::overcast(),
            Self::incoming_storm(),
            Self::heavy_storm(),
            Self::post_storm_calm(),
            Self::fog_morning(),
            Self::windy_overcast(),
        ]
    }
}
