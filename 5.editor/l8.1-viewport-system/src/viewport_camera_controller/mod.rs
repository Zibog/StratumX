// Viewport Camera Controller - Free camera movement for editor

#[cfg(feature = "desktop")]
pub mod focus;
#[cfg(feature = "desktop")]
pub mod input;
#[cfg(feature = "desktop")]
pub mod movement;

#[cfg(not(feature = "desktop"))]
pub struct ViewportCameraController;

#[cfg(not(feature = "desktop"))]
impl ViewportCameraController {
    pub fn new() -> Self {
        Self
    }
}

#[cfg(feature = "desktop")]
pub struct ViewportCameraController {
    position: [f32; 3],
    yaw: f32,
    pitch: f32,
    speed: f32,
    boost_multiplier: f32,
    mouse_sensitivity: f32,
    mouse_captured: bool,
    last_mouse_pos: Option<egui::Pos2>,
}

#[cfg(feature = "desktop")]
impl ViewportCameraController {
    pub fn new() -> Self {
        Self {
            position: [25.0, 8.0, -15.0],
            yaw: 0.0,
            pitch: -0.2,
            speed: 10.0,
            boost_multiplier: 3.0,
            mouse_sensitivity: 0.003,
            mouse_captured: false,
            last_mouse_pos: None,
        }
    }

    pub fn update(&mut self, ui: &egui::Ui, delta_time: f32) {
        input::handle_mouse_capture(ui, &mut self.mouse_captured, &mut self.last_mouse_pos);
        input::handle_mouse_look(
            ui,
            self.mouse_captured,
            &mut self.last_mouse_pos,
            &mut self.yaw,
            &mut self.pitch,
            self.mouse_sensitivity,
        );

        let (forward, right, up) = movement::calculate_direction_vectors(self.yaw, self.pitch);

        let current_speed = if ui.input(|i| i.modifiers.shift) {
            self.speed * self.boost_multiplier
        } else {
            self.speed
        };

        let move_speed = current_speed * delta_time;

        movement::apply_wasd_movement(ui, &mut self.position, forward, right, move_speed);
        movement::apply_vertical_movement(ui, &mut self.position, up, move_speed);

        if ui.input(|i| i.key_pressed(egui::Key::F)) {
            self.focus_point([25.0, 0.0, 25.0]);
        }

        if ui.input(|i| i.key_pressed(egui::Key::R)) {
            self.reset();
        }

        input::handle_scroll_speed(ui, &mut self.speed);
    }

    pub fn position(&self) -> [f32; 3] {
        self.position
    }

    pub fn look_at(&self) -> [f32; 3] {
        focus::calculate_look_at(self.position, self.yaw, self.pitch)
    }

    pub fn focus_point(&mut self, point: [f32; 3]) {
        focus::focus_on_point(point, self.position, &mut self.yaw, &mut self.pitch);
    }

    pub fn reset(&mut self) {
        self.position = [25.0, 8.0, -15.0];
        self.yaw = 0.0;
        self.pitch = -0.2;
        self.speed = 10.0;
    }

    pub fn is_mouse_captured(&self) -> bool {
        self.mouse_captured
    }
}

impl Default for ViewportCameraController {
    fn default() -> Self {
        Self::new()
    }
}
