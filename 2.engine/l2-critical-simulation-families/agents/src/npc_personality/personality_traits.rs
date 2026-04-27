use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PersonalityTraits {
    pub aggression: f32,
    pub greed: f32,
    pub loyalty: f32,
    pub courage: f32,
    pub discipline: f32,
    pub sociability: f32,
}

impl PersonalityTraits {
    pub fn new(
        aggression: f32,
        greed: f32,
        loyalty: f32,
        courage: f32,
        discipline: f32,
        sociability: f32,
    ) -> Self {
        Self {
            aggression: aggression.clamp(0.0, 1.0),
            greed: greed.clamp(0.0, 1.0),
            loyalty: loyalty.clamp(0.0, 1.0),
            courage: courage.clamp(0.0, 1.0),
            discipline: discipline.clamp(0.0, 1.0),
            sociability: sociability.clamp(0.0, 1.0),
        }
    }

    pub fn peaceful() -> Self {
        Self::new(0.2, 0.3, 0.7, 0.5, 0.6, 0.7)
    }

    pub fn aggressive() -> Self {
        Self::new(0.8, 0.6, 0.4, 0.7, 0.5, 0.4)
    }

    pub fn criminal() -> Self {
        Self::new(0.7, 0.8, 0.3, 0.6, 0.3, 0.5)
    }
}
