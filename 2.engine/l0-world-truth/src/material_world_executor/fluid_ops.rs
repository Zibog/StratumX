// Fluid operations - hydrology

use super::executor::MaterialWorldExecutor;
use engine_material::{FluidContainer, HydrologyState, Leak, Rainfall};

impl MaterialWorldExecutor {
    pub fn create_hydrology_container(
        &mut self,
        position: [f32; 3],
        capacity_liters: f32,
        height_m: f32,
        cross_section_m2: f32,
    ) -> usize {
        let container = FluidContainer::new(position, capacity_liters, height_m, cross_section_m2);
        let state = HydrologyState::new(container);
        self.hydrology_states.push(state);
        self.hydrology_states.len() - 1
    }

    pub fn add_leak_to_container(
        &mut self,
        container_index: usize,
        position: [f32; 3],
        hole_diameter_mm: f32,
    ) -> bool {
        if let Some(state) = self.hydrology_states.get_mut(container_index) {
            let leak = Leak::new(position, hole_diameter_mm);
            state.add_leak(leak);
            true
        } else {
            false
        }
    }

    pub fn set_rainfall_on_container(
        &mut self,
        container_index: usize,
        intensity_mm_per_hour: f32,
    ) -> bool {
        if let Some(state) = self.hydrology_states.get_mut(container_index) {
            state.rainfall = Some(Rainfall::new(
                intensity_mm_per_hour,
                state.container.cross_section_m2,
            ));
            true
        } else {
            false
        }
    }

    pub fn get_hydrology_state(&self, index: usize) -> Option<&HydrologyState> {
        self.hydrology_states.get(index)
    }
}
