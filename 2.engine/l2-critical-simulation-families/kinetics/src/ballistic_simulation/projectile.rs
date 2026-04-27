// Projectile management

use crate::{ProjectileProfile, ProjectileProfileId, ProjectileState, WeaponProfile};

pub struct BallisticSimulator {
    pub(crate) projectile_profiles: Vec<ProjectileProfile>,
    pub(crate) weapon_profiles: Vec<WeaponProfile>,
}

impl BallisticSimulator {
    pub fn new() -> Self {
        Self {
            projectile_profiles: Vec::new(),
            weapon_profiles: Vec::new(),
        }
    }

    pub fn register_projectile(&mut self, profile: ProjectileProfile) {
        self.projectile_profiles.push(profile);
    }

    pub fn register_weapon(&mut self, profile: WeaponProfile) {
        self.weapon_profiles.push(profile);
    }

    pub fn projectile_profile(&self, id: ProjectileProfileId) -> Option<&ProjectileProfile> {
        self.projectile_profiles.iter().find(|p| p.id == id)
    }

    pub fn spawn_projectile(
        &self,
        profile_id: ProjectileProfileId,
        position: [f32; 3],
        direction: [f32; 3],
    ) -> Option<ProjectileState> {
        let profile = self.projectile_profile(profile_id)?;
        let dir_len = (direction[0] * direction[0]
            + direction[1] * direction[1]
            + direction[2] * direction[2])
            .sqrt();
        let normalized = [
            direction[0] / dir_len,
            direction[1] / dir_len,
            direction[2] / dir_len,
        ];
        let velocity = [
            normalized[0] * profile.muzzle_velocity_m_s,
            normalized[1] * profile.muzzle_velocity_m_s,
            normalized[2] * profile.muzzle_velocity_m_s,
        ];
        Some(ProjectileState {
            profile_id,
            position,
            velocity,
            time_alive_s: 0.0,
        })
    }
}

impl Default for BallisticSimulator {
    fn default() -> Self {
        Self::new()
    }
}
