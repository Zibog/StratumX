use serde::{Deserialize, Serialize};

use super::creature_profile::{CreatureProfile, CreatureSpecies, MigrationReason};
use super::migration::MigrationState;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreatureEcologyState {
    pub creature_id: u32,
    pub profile: CreatureProfile,
    pub migration: MigrationState,
}

impl CreatureEcologyState {
    pub fn new(creature_id: u32, species: CreatureSpecies, position: [f32; 3]) -> Self {
        Self {
            creature_id,
            profile: CreatureProfile::new(species),
            migration: MigrationState::new(position),
        }
    }

    pub fn update(&mut self, delta_hours: f32, weather_severity: f32) {
        self.profile.hunger_level = (self.profile.hunger_level + delta_hours * 3.0).min(100.0);

        if weather_severity > 0.7 {
            self.profile.fear_level =
                (self.profile.fear_level + weather_severity * 10.0).min(100.0);
        } else {
            self.profile.fear_level = (self.profile.fear_level - delta_hours * 2.0).max(0.0);
        }

        if !self.migration.active && self.profile.should_migrate() {
            if let Some(reason) = self.profile.determine_migration_reason() {
                let destination = self.choose_destination(reason);
                self.migration.start_migration(destination, reason);
            }
        }
    }

    fn choose_destination(&self, reason: MigrationReason) -> [f32; 3] {
        let current = self.migration.current_position;

        match reason {
            MigrationReason::Hunger => [0.0, 0.0, 0.0],
            MigrationReason::Predator => [current[0] + 100.0, current[1], current[2] + 100.0],
            MigrationReason::Weather => [-50.0, 0.0, -50.0],
            MigrationReason::Territory => [current[0] + 50.0, current[1], current[2] - 50.0],
        }
    }

    pub fn feed(&mut self, amount: f32) {
        self.profile.hunger_level = (self.profile.hunger_level - amount).max(0.0);
    }

    pub fn scare(&mut self, intensity: f32) {
        self.profile.fear_level = (self.profile.fear_level + intensity).min(100.0);
    }
}
