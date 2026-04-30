// Desktop App - Launch-critical thin shell only

pub mod app_commands;
pub mod app_diagnostics;
pub mod app_panels;
pub mod app_state;
pub mod app_update;
pub mod app_viewport;
pub mod editor_app;

pub use editor_app::EditorApp;

use crate::shell_bootstrap::ShellBootstrap;
use eframe::egui;

pub fn run_desktop_app() {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1600.0, 900.0])
            .with_title("StratumX Editor - Production"),
        renderer: eframe::Renderer::Wgpu,
        ..Default::default()
    };

    let _ = eframe::run_native(
        "StratumX Editor",
        options,
        Box::new(|cc| {
            let mut bootstrap = ShellBootstrap::new();
            match bootstrap.bootstrap() {
                Ok(host) => Ok(Box::new(EditorApp::new(host, cc))),
                Err(e) => {
                    eprintln!("WARNING: Bootstrap failed: {}", e);
                    eprintln!("Starting editor with empty host as fallback...");
                    // Fallback: create a minimal host so editor still opens
                    let mut host = crate::editor_host::EditorHost::default();
                    // Try startup but don't fail if it doesn't work
                    let _ = host.initialize();
                    let _ = host.startup();
                    Ok(Box::new(EditorApp::new(host, cc)))
                }
            }
        }),
    );
}
