// Destruction operations - structure failure

use super::executor::MaterialWorldExecutor;
use engine_material::{DestructionResponse, StructureType};

impl MaterialWorldExecutor {
    pub fn apply_destruction(
        &mut self,
        structure_type: StructureType,
        impact_position: [f32; 3],
        impact_energy_j: f32,
        impact_direction: [f32; 3],
    ) -> usize {
        let response = DestructionResponse::from_impact(
            structure_type,
            impact_position,
            impact_energy_j,
            impact_direction,
        );
        self.destruction_responses.push(response);
        self.destruction_responses.len() - 1
    }

    pub fn get_destruction_response(&self, index: usize) -> Option<&DestructionResponse> {
        self.destruction_responses.get(index)
    }
}
