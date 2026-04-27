// Camera movement logic

#[cfg(feature = "desktop")]
use egui::Key;

#[cfg(feature = "desktop")]
pub fn calculate_direction_vectors(yaw: f32, pitch: f32) -> ([f32; 3], [f32; 3], [f32; 3]) {
    let forward = [
        yaw.cos() * pitch.cos(),
        pitch.sin(),
        yaw.sin() * pitch.cos(),
    ];

    let right = [
        (yaw - std::f32::consts::FRAC_PI_2).cos(),
        0.0,
        (yaw - std::f32::consts::FRAC_PI_2).sin(),
    ];

    let up = [0.0, 1.0, 0.0];

    (forward, right, up)
}

#[cfg(feature = "desktop")]
pub fn apply_wasd_movement(
    ui: &egui::Ui,
    position: &mut [f32; 3],
    forward: [f32; 3],
    right: [f32; 3],
    move_speed: f32,
) {
    if ui.input(|i| i.key_down(Key::W)) {
        position[0] += forward[0] * move_speed;
        position[1] += forward[1] * move_speed;
        position[2] += forward[2] * move_speed;
    }
    if ui.input(|i| i.key_down(Key::S)) {
        position[0] -= forward[0] * move_speed;
        position[1] -= forward[1] * move_speed;
        position[2] -= forward[2] * move_speed;
    }
    if ui.input(|i| i.key_down(Key::A)) {
        position[0] -= right[0] * move_speed;
        position[1] -= right[1] * move_speed;
        position[2] -= right[2] * move_speed;
    }
    if ui.input(|i| i.key_down(Key::D)) {
        position[0] += right[0] * move_speed;
        position[1] += right[1] * move_speed;
        position[2] += right[2] * move_speed;
    }
}

#[cfg(feature = "desktop")]
pub fn apply_vertical_movement(
    ui: &egui::Ui,
    position: &mut [f32; 3],
    up: [f32; 3],
    move_speed: f32,
) {
    if ui.input(|i| i.key_down(Key::Q)) {
        position[0] -= up[0] * move_speed;
        position[1] -= up[1] * move_speed;
        position[2] -= up[2] * move_speed;
    }
    if ui.input(|i| i.key_down(Key::E)) {
        position[0] += up[0] * move_speed;
        position[1] += up[1] * move_speed;
        position[2] += up[2] * move_speed;
    }
}
