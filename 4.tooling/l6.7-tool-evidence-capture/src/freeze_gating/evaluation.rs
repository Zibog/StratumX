use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HardwareFloorResult {
    pub pack_id: String,
    pub scenario_id: String,
    pub frame_time_ms: f32,
    pub memory_usage_mb: f32,
    pub texture_pressure: TexturePressureLevel,
    pub degrade_ladder: Vec<DegradeLadderStep>,
    pub timestamp: u64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TexturePressureLevel {
    Healthy,
    Elevated,
    Critical,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DegradeLadderStep {
    pub step_name: String,
    pub description: String,
    pub frame_time_impact_ms: f32,
    pub memory_savings_mb: f32,
    pub visual_quality_impact: String,
}

impl HardwareFloorResult {
    pub fn new(pack_id: String, scenario_id: String) -> Self {
        Self {
            pack_id,
            scenario_id,
            frame_time_ms: 0.0,
            memory_usage_mb: 0.0,
            texture_pressure: TexturePressureLevel::Healthy,
            degrade_ladder: Vec::new(),
            timestamp: 0,
        }
    }

    pub fn add_degrade_step(&mut self, step: DegradeLadderStep) {
        self.degrade_ladder.push(step);
    }

    pub fn is_within_budget(&self, target_frame_time_ms: f32, target_memory_mb: f32) -> bool {
        self.frame_time_ms <= target_frame_time_ms && self.memory_usage_mb <= target_memory_mb
    }

    pub fn calculate_degrade_savings(&self) -> (f32, f32) {
        let total_frame_savings: f32 = self
            .degrade_ladder
            .iter()
            .map(|s| s.frame_time_impact_ms)
            .sum();
        let total_memory_savings: f32 = self
            .degrade_ladder
            .iter()
            .map(|s| s.memory_savings_mb)
            .sum();
        (total_frame_savings, total_memory_savings)
    }
}
