use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CreatureSpecies {
    Deer,
    Wolf,
    Bear,
    Rabbit,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MigrationReason {
    Hunger,
    Predator,
    Weather,
    Territory,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreatureProfile {
    pub species: CreatureSpecies,
    pub hunger_level: f32,
    pub fear_level: f32,
    pub territory_pressure: f32,
}

impl CreatureProfile {
    pub fn new(species: CreatureSpecies) -> Self {
        Self {
            species,
            hunger_level: 20.0,
            fear_level: 10.0,
            territory_pressure: 15.0,
        }
    }

    pub fn should_migrate(&self) -> bool {
        self.hunger_level > 70.0 || self.fear_level > 80.0 || self.territory_pressure > 75.0
    }

    pub fn determine_migration_reason(&self) -> Option<MigrationReason> {
        if self.fear_level > 80.0 {
            Some(MigrationReason::Predator)
        } else if self.hunger_level > 70.0 {
            Some(MigrationReason::Hunger)
        } else if self.territory_pressure > 75.0 {
            Some(MigrationReason::Territory)
        } else {
            None
        }
    }
}
