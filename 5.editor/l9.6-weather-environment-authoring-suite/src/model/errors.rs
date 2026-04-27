use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum WeatherError {
    InvalidConfiguration(String),
    UnknownTheater,
    UnknownStormFront,
    StormFrontError(String),
    ConfigError(String),
    UpdateError(String),
}

impl std::fmt::Display for WeatherError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WeatherError::InvalidConfiguration(msg) => {
                write!(f, "Invalid configuration: {}", msg)
            }
            WeatherError::UnknownTheater => write!(f, "Unknown climate theater"),
            WeatherError::UnknownStormFront => write!(f, "Unknown storm front"),
            WeatherError::StormFrontError(msg) => write!(f, "Storm front error: {}", msg),
            WeatherError::ConfigError(msg) => write!(f, "Configuration error: {}", msg),
            WeatherError::UpdateError(msg) => write!(f, "Update error: {}", msg),
        }
    }
}

impl std::error::Error for WeatherError {}
