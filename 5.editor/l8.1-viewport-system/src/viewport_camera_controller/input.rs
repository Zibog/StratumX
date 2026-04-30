// Camera input handling

#[cfg(feature = "desktop")]
pub fn handle_mouse_capture(
    ui: &egui::Ui,
    mouse_captured: &mut bool,
    last_mouse_pos: &mut Option<egui::Pos2>,
) {
    if ui.input(|i| i.pointer.button_pressed(egui::PointerButton::Secondary)) {
        *mouse_captured = true;
        *last_mouse_pos = ui.input(|i| i.pointer.hover_pos());
    }

    if ui.input(|i| i.pointer.button_released(egui::PointerButton::Secondary)) {
        *mouse_captured = false;
        *last_mouse_pos = None;
    }
}

#[cfg(feature = "desktop")]
pub fn handle_mouse_look(
    ui: &egui::Ui,
    mouse_captured: bool,
    last_mouse_pos: &mut Option<egui::Pos2>,
    yaw: &mut f32,
    pitch: &mut f32,
    mouse_sensitivity: f32,
) {
    if mouse_captured {
        if let Some(current_pos) = ui.input(|i| i.pointer.hover_pos()) {
            if let Some(last_pos) = *last_mouse_pos {
                let delta = current_pos - last_pos;
                *yaw -= delta.x * mouse_sensitivity;
                *pitch -= delta.y * mouse_sensitivity;

                *pitch = pitch.clamp(
                    -std::f32::consts::FRAC_PI_2 + 0.1,
                    std::f32::consts::FRAC_PI_2 - 0.1,
                );
            }
            *last_mouse_pos = Some(current_pos);
        }
    }
}

#[cfg(feature = "desktop")]
pub fn handle_scroll_speed(ui: &egui::Ui, speed: &mut f32) {
    let scroll_delta = ui.input(|i| i.smooth_scroll_delta.y);
    if scroll_delta != 0.0 {
        *speed = (*speed + scroll_delta * 0.1).clamp(1.0, 100.0);
    }
}
