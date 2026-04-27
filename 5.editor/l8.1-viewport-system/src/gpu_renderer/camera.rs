// Camera Operations

use super::types::{CameraUniform, GpuViewportRenderer};

impl GpuViewportRenderer {
    pub fn update_camera(&mut self, camera_pos: [f32; 3], look_at: [f32; 3], aspect: f32) {
        let eye = glam::Vec3::from_array(camera_pos);
        let target = glam::Vec3::from_array(look_at);
        let up = glam::Vec3::Y;

        let view = glam::Mat4::look_at_rh(eye, target, up);

        let fov = 75.0_f32.to_radians();
        let near = 0.1;
        let far = 1000.0;

        let proj = glam::Mat4::perspective_rh(fov, aspect, near, far);
        let view_proj = proj * view;

        let camera_uniform = CameraUniform {
            view_proj: view_proj.to_cols_array_2d(),
            camera_pos: [camera_pos[0], camera_pos[1], camera_pos[2], 1.0],
        };

        self.queue.write_buffer(
            &self.camera_buffer,
            0,
            bytemuck::cast_slice(&[camera_uniform]),
        );
    }
}
