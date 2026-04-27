// Material World Executor - главный тип и update loop

use engine_material::{
    CombustibleObject, DestructionResponse, HydrologyState, SmokeSystem, TerrainBlastResponse,
};

#[derive(Debug)]
pub struct MaterialWorldExecutor {
    pub terrain_responses: Vec<TerrainBlastResponse>,
    pub destruction_responses: Vec<DestructionResponse>,
    pub hydrology_states: Vec<HydrologyState>,
    pub combustible_objects: Vec<CombustibleObject>,
    pub smoke_system: SmokeSystem,
}

impl MaterialWorldExecutor {
    pub fn new(_latitude_deg: f32) -> Self {
        Self {
            terrain_responses: Vec::new(),
            destruction_responses: Vec::new(),
            hydrology_states: Vec::new(),
            combustible_objects: Vec::new(),
            smoke_system: SmokeSystem::new(),
        }
    }

    pub fn update(&mut self, delta_time_sec: f32, rain_intensity: f32, wind_velocity: [f32; 3]) {
        use engine_material::Rainfall;

        for state in &mut self.hydrology_states {
            if rain_intensity > 0.0 {
                state.rainfall = Some(Rainfall::new(
                    rain_intensity,
                    state.container.cross_section_m2,
                ));
            } else {
                state.rainfall = None;
            }
            state.update(delta_time_sec);
        }

        let ambient_temp = 20.0;
        let current_time = 0.0;
        for obj in &mut self.combustible_objects {
            if rain_intensity > 0.0 {
                obj.apply_rain(rain_intensity, current_time);
            }

            obj.update(delta_time_sec, ambient_temp);

            if obj.fire.burning {
                self.smoke_system.emit_smoke(obj.position, 1.0);
            }
        }

        self.smoke_system.wind_velocity = wind_velocity;
        self.smoke_system.update(delta_time_sec);
    }

    pub fn get_rainfall_at_position(&self, _position: [f32; 2]) -> f32 {
        0.0
    }
}
