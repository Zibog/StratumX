// Mesh Generation

use super::types::Vertex;
use engine_world::WorldState;
pub(crate) fn create_empty_terrain_mesh() -> (Vec<Vertex>, Vec<u16>) {
    let vertices = vec![
        Vertex {
            position: [0.0, 0.0, 0.0],
            color: [0.4, 0.3, 0.2],
            uv: [0.0, 0.0],
        },
        Vertex {
            position: [1.0, 0.0, 0.0],
            color: [0.4, 0.3, 0.2],
            uv: [1.0, 0.0],
        },
        Vertex {
            position: [0.0, 0.0, 1.0],
            color: [0.4, 0.3, 0.2],
            uv: [0.0, 1.0],
        },
    ];
    let indices = vec![0, 1, 2];
    (vertices, indices)
}
pub(crate) fn create_terrain_mesh_from_world(world: &WorldState) -> (Vec<Vertex>, Vec<u16>) {
    if let Some(scene) = world.proof_region_scene() {
        let terrain = &scene.terrain;
        let res_x = terrain.resolution[0] as usize;
        let res_z = terrain.resolution[1] as usize;

        if terrain.height_samples.is_empty() || terrain.height_samples.len() != res_x * res_z {
            return create_empty_terrain_mesh();
        }

        let world_size_x = terrain.world_size[0];
        let world_size_z = terrain.world_size[1];
        let cell_size_x = world_size_x / (res_x - 1) as f32;
        let cell_size_z = world_size_z / (res_z - 1) as f32;

        let mut vertices = Vec::new();
        let mut indices = Vec::new();

        for z in 0..res_z {
            for x in 0..res_x {
                let idx = z * res_x + x;
                let height = terrain.height_samples[idx];

                let pos_x = terrain.origin[0] + x as f32 * cell_size_x - world_size_x / 2.0;
                let pos_z = terrain.origin[2] + z as f32 * cell_size_z - world_size_z / 2.0;
                let pos_y = terrain.origin[1] + height;

                let height_factor = (height / 100.0).clamp(0.0, 1.0);
                let color = [0.3 + height_factor * 0.3, 0.4 + height_factor * 0.2, 0.2];

                // Generate UV coordinates normalized to terrain extents
                let u = x as f32 / (res_x - 1) as f32;
                let v = z as f32 / (res_z - 1) as f32;

                vertices.push(Vertex {
                    position: [pos_x, pos_y, pos_z],
                    color,
                    uv: [u, v],
                });
            }
        }

        for z in 0..(res_z - 1) {
            for x in 0..(res_x - 1) {
                let top_left = (z * res_x + x) as u16;
                let top_right = top_left + 1;
                let bottom_left = ((z + 1) * res_x + x) as u16;
                let bottom_right = bottom_left + 1;

                indices.push(top_left);
                indices.push(bottom_left);
                indices.push(top_right);

                indices.push(top_right);
                indices.push(bottom_left);
                indices.push(bottom_right);
            }
        }

        (vertices, indices)
    } else {
        create_empty_terrain_mesh()
    }
}
pub(crate) fn create_sun_mesh() -> (Vec<Vertex>, Vec<u16>) {
    let sun_size = 20.0;
    let sun_color = [1.0, 0.95, 0.8];

    let vertices = vec![
        Vertex {
            position: [-sun_size, -sun_size, 0.0],
            color: sun_color,
            uv: [0.0, 0.0],
        },
        Vertex {
            position: [sun_size, -sun_size, 0.0],
            color: sun_color,
            uv: [1.0, 0.0],
        },
        Vertex {
            position: [sun_size, sun_size, 0.0],
            color: sun_color,
            uv: [1.0, 1.0],
        },
        Vertex {
            position: [-sun_size, sun_size, 0.0],
            color: sun_color,
            uv: [0.0, 1.0],
        },
    ];

    let indices = vec![0, 1, 2, 0, 2, 3];

    (vertices, indices)
}

pub(crate) fn create_cloud_meshes(
    world: &WorldState,
    camera_pos: [f32; 3],
) -> (Vec<Vertex>, Vec<u16>) {
    let mut vertices = Vec::new();
    let mut indices = Vec::new();

    if let Some(scene) = world.proof_region_scene() {
        let sky = &scene.sky;

        let cloud_coverage = match sky.weather_director.target_regime {
            engine_material::WeatherRegime::Clear => 0.1,
            engine_material::WeatherRegime::Scattered => 0.3,
            engine_material::WeatherRegime::Overcast => 0.7,
            engine_material::WeatherRegime::IncomingStorm => 0.6,
            engine_material::WeatherRegime::HeavyStorm => 0.8,
            engine_material::WeatherRegime::PostStormCalm => 0.4,
            engine_material::WeatherRegime::FogMorning => 0.5,
            engine_material::WeatherRegime::WindyOvercast => 0.7,
        };

        let cloud_count = (cloud_coverage * 20.0) as usize;
        let cloud_height = 50.0;
        let cloud_size = 30.0;

        for i in 0..cloud_count {
            let angle = (i as f32 / cloud_count as f32) * std::f32::consts::TAU;
            let radius = 40.0 + (i as f32 * 3.0);

            let cloud_x = camera_pos[0] + angle.cos() * radius;
            let cloud_y = cloud_height;
            let cloud_z = camera_pos[2] + angle.sin() * radius;

            let cloud_color = [0.9, 0.9, 0.95];
            let base_idx = vertices.len() as u16;

            vertices.push(Vertex {
                position: [cloud_x - cloud_size, cloud_y - cloud_size, cloud_z],
                color: cloud_color,
                uv: [0.0, 0.0],
            });
            vertices.push(Vertex {
                position: [cloud_x + cloud_size, cloud_y - cloud_size, cloud_z],
                color: cloud_color,
                uv: [1.0, 0.0],
            });
            vertices.push(Vertex {
                position: [cloud_x + cloud_size, cloud_y + cloud_size, cloud_z],
                color: cloud_color,
                uv: [1.0, 1.0],
            });
            vertices.push(Vertex {
                position: [cloud_x - cloud_size, cloud_y + cloud_size, cloud_z],
                color: cloud_color,
                uv: [0.0, 1.0],
            });

            indices.extend_from_slice(&[
                base_idx,
                base_idx + 1,
                base_idx + 2,
                base_idx,
                base_idx + 2,
                base_idx + 3,
            ]);
        }
    }

    (vertices, indices)
}
