pub const MAX_RELEASED_SEGMENTS_PER_SHOT: usize = 16;
pub const MAX_DEBRIS_PROXIES_PER_SHOT: usize = 8;
pub const MAX_DUST_EVENTS_PER_SHOT: usize = 4;

#[derive(Debug, Clone, PartialEq)]
pub struct LayerImpactInput {
    pub layer_index: u8,
    pub energy_absorbed_j: f32,
    pub projectile_diameter_mm: f32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DamageApplicationResult {
    pub cracked_segments: Vec<u16>,
    pub released_segments: Vec<u16>,
    pub debris_count: usize,
    pub dust_events: usize,
}
