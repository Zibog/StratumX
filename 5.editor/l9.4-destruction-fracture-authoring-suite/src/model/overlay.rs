use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct OverlayId(pub u64);

impl OverlayId {
    pub fn new(raw: u64) -> Self {
        Self(raw)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum OverlayType {
    FracturePattern { density: f32 },
    WeakPoint { position: [f32; 3], radius: f32 },
    ProtectedZone { bounds: [f32; 6] },
    Custom { name: String, data: String },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Overlay {
    pub overlay_type: OverlayType,
    pub enabled: bool,
    pub priority: i32,
}

impl Overlay {
    pub fn new(overlay_type: OverlayType) -> Self {
        Self {
            overlay_type,
            enabled: true,
            priority: 0,
        }
    }

    pub fn validate(&self) -> Result<(), String> {
        match &self.overlay_type {
            OverlayType::FracturePattern { density } => {
                if *density <= 0.0 || *density > 1.0 {
                    return Err("Fracture pattern density must be between 0 and 1".to_string());
                }
            }
            OverlayType::WeakPoint { radius, .. } => {
                if *radius <= 0.0 {
                    return Err("Weak point radius must be positive".to_string());
                }
            }
            OverlayType::ProtectedZone { bounds } => {
                if bounds[3] <= bounds[0] || bounds[4] <= bounds[1] || bounds[5] <= bounds[2] {
                    return Err("Protected zone bounds are invalid".to_string());
                }
            }
            OverlayType::Custom { name, .. } => {
                if name.is_empty() {
                    return Err("Custom overlay name cannot be empty".to_string());
                }
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DisabledReason {
    FieldDisabled,
    InvalidConfiguration(String),
    MissingDependency(String),
    PerformanceLimit,
    Custom(String),
}

impl DisabledReason {
    pub fn description(&self) -> String {
        match self {
            Self::FieldDisabled => "Destruction field is disabled".to_string(),
            Self::InvalidConfiguration(msg) => format!("Invalid configuration: {}", msg),
            Self::MissingDependency(dep) => format!("Missing dependency: {}", dep),
            Self::PerformanceLimit => "Performance limit reached".to_string(),
            Self::Custom(msg) => msg.clone(),
        }
    }
}
