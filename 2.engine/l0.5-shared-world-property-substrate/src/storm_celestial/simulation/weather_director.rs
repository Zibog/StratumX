use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WeatherRegime {
    Clear,
    Scattered,
    Overcast,
    IncomingStorm,
    HeavyStorm,
    PostStormCalm,
    FogMorning,
    WindyOvercast,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WeatherDirectorState {
    pub target_regime: WeatherRegime,
    pub transition_progress: f32,
    pub authoring_lock: bool,
    pub storm_bias: f32,
    pub fog_bias: f32,
    pub rain_bias: f32,
}
