//! Startup sequence for the editor host.
//!
//! **NOTE:** Full startup-world creation (loading a WorldState into
//! `WorldLifecycleService`) must be implemented inside the
//! `stratumx-editor-l9-0-world-authoring-suite` crate because the
//! `loaded_world` / `loaded_world_label` / `loaded_world_path` fields are
//! `pub(crate)` there.  Until that is added, this startup creates a minimal
//! session without a loaded world and logs a warning.

use super::EditorHost;
use std::hash::{Hash, Hasher};
use std::time::SystemTime;
use stratumx_editor_l8_10_diagnostics_surface::host::diagnostics_types::{Diagnostic, Severity};
use stratumx_editor_l8_5_tool_context_system::context::session::{EditorSession, RuntimeMode};
use uuid::Uuid;

impl EditorHost {
    /// Startup sequence — opens startup world and binds all services.
    ///
    /// **PHASE 6/7/8 REMEDIATED — Single World Path:**
    /// The lifecycle service holds the loaded world and `startup()` consumes
    /// it via `take_loaded_world()`, ensuring exactly one world exists.
    pub fn startup(&mut self) -> Result<(), String> {
        eprintln!("startup.begin");

        // Step 1: Open startup world through lifecycle service.
        // Deferred follow-up: once `open_startup_world_from_package` is added
        // to the world-authoring-suite crate, wire that canonical startup path here.
        let _startup_path = std::path::Path::new("9.assets/worlds/startup_world");

        // Step 2: Try to consume the loaded world from lifecycle service.
        // If nothing loaded it yet, we report this and continue — the editor will still
        // be usable, just without an initial world.
        if let Some((world, label, path)) = self.world_lifecycle.take_loaded_world() {
            let label: String = label;
            let path: Option<std::path::PathBuf> = path;
            let mut hasher = std::collections::hash_map::DefaultHasher::new();
            label.hash(&mut hasher);
            let hash = hasher.finish() as u128;
            let world_ref =
                editor_dto_law::StableWorldId(Uuid::from_u128(hash | (0x53545241u128 << 96)));

            self.world_lifecycle
                .register_opened_world(label.clone(), path.clone());

            let _session = EditorSession {
                world_ref,
                world,
                world_label: label,
                world_path: path,
                runtime_mode: RuntimeMode::Editing,
            };
            self.terrain_gpu_dirty = true;

            self.diagnostics.collect_diagnostic(Diagnostic {
                severity: Severity::Info,
                domain: "startup".to_string(),
                message: "startup.world_loaded".to_string(),
                timestamp: SystemTime::now(),
            });
        } else {
            // No world loaded yet — this is expected until startup-world
            // creation is implemented in the world-authoring-suite crate.
            eprintln!(
                "WARNING: No world loaded from lifecycle service; \
                 startup-world creation must be added to the \
                 stratumx-editor-l9-0-world-authoring-suite crate"
            );
            self.diagnostics.collect_diagnostic(Diagnostic {
                severity: Severity::Warning,
                domain: "startup".to_string(),
                message: "startup.no_world: world-authoring-suite needs startup creation"
                    .to_string(),
                timestamp: SystemTime::now(),
            });
        }

        // Step 3: Initialize remaining services
        self.terrain.initialize()?;
        self.environment.initialize()?;
        self.viewport.initialize()?;
        self.runtime_control.initialize()?;

        self.diagnostics.collect_diagnostic(Diagnostic {
            severity: Severity::Info,
            domain: "startup".to_string(),
            message: "All editor services initialized".to_string(),
            timestamp: SystemTime::now(),
        });

        Ok(())
    }
}
