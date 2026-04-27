// Camera focus operations

pub fn focus_on_point(point: [f32; 3], position: [f32; 3], yaw: &mut f32, pitch: &mut f32) {
    let dx = point[0] - position[0];
    let dy = point[1] - position[1];
    let dz = point[2] - position[2];

    let horizontal_dist = (dx * dx + dz * dz).sqrt();

    *yaw = dz.atan2(dx);
    *pitch = dy.atan2(horizontal_dist);

    *pitch = pitch.clamp(
        -std::f32::consts::FRAC_PI_2 + 0.1,
        std::f32::consts::FRAC_PI_2 - 0.1,
    );
}

pub fn calculate_look_at(position: [f32; 3], yaw: f32, pitch: f32) -> [f32; 3] {
    let forward = [
        yaw.cos() * pitch.cos(),
        pitch.sin(),
        yaw.sin() * pitch.cos(),
    ];

    [
        position[0] + forward[0],
        position[1] + forward[1],
        position[2] + forward[2],
    ]
}
