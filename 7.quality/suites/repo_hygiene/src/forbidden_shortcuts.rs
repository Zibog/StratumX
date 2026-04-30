use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ForbiddenShortcut {
    MutatingHostCall { method: String },
    ReadOnlyHostQuery { method: String },
    ExecuteBridge,
    DirectWorldStateMutation,
    DirectSdkEngineMutation,
    // Layer boundary violations
    EditorImportsEngine { imported_type: String },
    ToolingImportsEditor { imported_type: String },
    AppContainsDomainLogic { pattern: String },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ForbiddenShortcutViolation {
    pub file: PathBuf,
    pub line: usize,
    pub shortcut: ForbiddenShortcut,
    pub snippet: String,
}

pub struct ForbiddenShortcutScanner {
    repo_root: PathBuf,
}

impl ForbiddenShortcutScanner {
    pub fn new(repo_root: PathBuf) -> Self {
        Self { repo_root }
    }

    /// Scan for layer boundary violations across the entire codebase
    pub fn scan_layer_boundaries(&self) -> Vec<ForbiddenShortcutViolation> {
        let mut violations = Vec::new();

        // Check editor layer doesn't import engine types directly
        violations.extend(self.scan_editor_to_engine_violations());

        // Check tooling layer doesn't import editor types
        violations.extend(self.scan_tooling_to_editor_violations());

        // Check apps don't contain domain logic
        violations.extend(self.scan_apps_domain_logic());

        violations
    }

    fn scan_editor_to_engine_violations(&self) -> Vec<ForbiddenShortcutViolation> {
        let mut violations = Vec::new();
        let editor_dir = self.repo_root.join("5.editor");

        if !editor_dir.exists() {
            return violations;
        }

        let mut editor_files = Vec::new();
        self.walk_rust_files(&editor_dir, &mut editor_files);

        for file in editor_files {
            violations.extend(self.scan_file_for_engine_imports(&file));
        }

        violations
    }

    fn scan_tooling_to_editor_violations(&self) -> Vec<ForbiddenShortcutViolation> {
        let mut violations = Vec::new();
        let tooling_dir = self.repo_root.join("4.tooling");

        if !tooling_dir.exists() {
            return violations;
        }

        let mut tooling_files = Vec::new();
        self.walk_rust_files(&tooling_dir, &mut tooling_files);

        for file in tooling_files {
            violations.extend(self.scan_file_for_editor_imports(&file));
        }

        violations
    }

    fn scan_apps_domain_logic(&self) -> Vec<ForbiddenShortcutViolation> {
        let mut violations = Vec::new();
        let apps_dir = self.repo_root.join("6.apps");

        if !apps_dir.exists() {
            return violations;
        }

        let mut app_files = Vec::new();
        self.walk_rust_files(&apps_dir, &mut app_files);

        for file in app_files {
            violations.extend(self.scan_file_for_domain_logic(&file));
        }

        violations
    }

    fn scan_file_for_engine_imports(&self, path: &Path) -> Vec<ForbiddenShortcutViolation> {
        let content = match fs::read_to_string(path) {
            Ok(content) => content,
            Err(_) => return Vec::new(),
        };

        let mut violations = Vec::new();

        for (index, line) in content.lines().enumerate() {
            let trimmed = line.trim();

            // Check for direct engine imports (use statements from 2.engine packages)
            if trimmed.starts_with("use ") && self.is_engine_import(trimmed) {
                let imported_type = self.extract_import_path(trimmed);
                violations.push(ForbiddenShortcutViolation {
                    file: path.to_path_buf(),
                    line: index + 1,
                    shortcut: ForbiddenShortcut::EditorImportsEngine {
                        imported_type: imported_type.clone(),
                    },
                    snippet: trimmed.to_string(),
                });
            }
        }

        violations
    }

    fn scan_file_for_editor_imports(&self, path: &Path) -> Vec<ForbiddenShortcutViolation> {
        let content = match fs::read_to_string(path) {
            Ok(content) => content,
            Err(_) => return Vec::new(),
        };

        let mut violations = Vec::new();

        for (index, line) in content.lines().enumerate() {
            let trimmed = line.trim();

            // Check for editor imports (use statements from 5.editor packages)
            if trimmed.starts_with("use ") && self.is_editor_import(trimmed) {
                let imported_type = self.extract_import_path(trimmed);
                violations.push(ForbiddenShortcutViolation {
                    file: path.to_path_buf(),
                    line: index + 1,
                    shortcut: ForbiddenShortcut::ToolingImportsEditor {
                        imported_type: imported_type.clone(),
                    },
                    snippet: trimmed.to_string(),
                });
            }
        }

        violations
    }

    fn scan_file_for_domain_logic(&self, path: &Path) -> Vec<ForbiddenShortcutViolation> {
        let content = match fs::read_to_string(path) {
            Ok(content) => content,
            Err(_) => return Vec::new(),
        };

        let mut violations = Vec::new();

        for (index, line) in content.lines().enumerate() {
            let trimmed = line.trim();

            // Check for domain logic patterns in apps
            // Domain logic indicators: complex business logic, state management, validation
            if self.is_domain_logic_pattern(trimmed) {
                let pattern = self.extract_domain_logic_pattern(trimmed);
                violations.push(ForbiddenShortcutViolation {
                    file: path.to_path_buf(),
                    line: index + 1,
                    shortcut: ForbiddenShortcut::AppContainsDomainLogic {
                        pattern: pattern.clone(),
                    },
                    snippet: trimmed.to_string(),
                });
            }
        }

        violations
    }

    pub fn is_engine_import(&self, line: &str) -> bool {
        // Check if the import is from a 2.engine package
        // Engine packages start with l-0, l0, l1, l2, l3, l4
        line.contains("::l_0") || 
        line.contains("::l0") || 
        line.contains("::l1") || 
        line.contains("::l2") || 
        line.contains("::l3") || 
        line.contains("::l4") ||
        // Also check for direct crate names that are engine packages
        (line.starts_with("use l") && (
            line.starts_with("use l_0") ||
            line.starts_with("use l0") ||
            line.starts_with("use l1") ||
            line.starts_with("use l2") ||
            line.starts_with("use l3") ||
            line.starts_with("use l4")
        ))
    }

    pub fn is_editor_import(&self, line: &str) -> bool {
        // Check if the import is from a 5.editor package
        // Editor packages start with l7, l8, l9, l10, l11
        line.contains("::l7") || 
        line.contains("::l8") || 
        line.contains("::l9") || 
        line.contains("::l10") || 
        line.contains("::l11") ||
        // Also check for direct crate names that are editor packages
        (line.starts_with("use l") && (
            line.starts_with("use l7") ||
            line.starts_with("use l8") ||
            line.starts_with("use l9") ||
            line.starts_with("use l10") ||
            line.starts_with("use l11")
        ))
    }

    pub fn is_domain_logic_pattern(&self, line: &str) -> bool {
        // Domain logic patterns that shouldn't be in apps
        // These are heuristics - apps should only have bootstrap/wiring

        // Skip comments and simple declarations
        if line.starts_with("//") || line.starts_with("/*") {
            return false;
        }

        // Skip UI event handlers - these are legitimate app concerns
        if line.contains("handle_shortcuts")
            || line.contains("handle_input")
            || line.contains("handle_event")
            || line.contains("handle_ui")
        {
            return false;
        }

        // Check for complex validation logic (not simple input validation)
        if line.contains("validate_") && line.contains("fn ") && !line.contains("validate_input") {
            return true;
        }

        // Check for business rule implementations
        if (line.contains("calculate_") || line.contains("compute_"))
            && line.contains("fn ")
            && !line.contains("calculate_layout")
        {
            return true;
        }

        // Check for state mutation logic (not simple setters or UI state)
        if line.contains("mut ")
            && (line.contains("process_command")
                || line.contains("apply_rules")
                || line.contains("execute_business"))
            && line.contains("fn ")
        {
            return true;
        }

        false
    }

    fn extract_import_path(&self, line: &str) -> String {
        // Extract the imported path from a use statement
        line.trim_start_matches("use ")
            .trim_end_matches(';')
            .split("::")
            .take(3)
            .collect::<Vec<_>>()
            .join("::")
    }

    fn extract_domain_logic_pattern(&self, line: &str) -> String {
        // Extract the function name or pattern
        if let Some(fn_pos) = line.find("fn ") {
            let after_fn = &line[fn_pos + 3..];
            if let Some(paren_pos) = after_fn.find('(') {
                return after_fn[..paren_pos].trim().to_string();
            }
        }
        "domain_logic".to_string()
    }

    fn walk_rust_files(&self, dir: &Path, files: &mut Vec<PathBuf>) {
        let entries = match fs::read_dir(dir) {
            Ok(entries) => entries,
            Err(_) => return,
        };

        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                if let Some(name) = path.file_name().and_then(|name| name.to_str()) {
                    if name.starts_with('.') || name == "target" {
                        continue;
                    }
                }
                self.walk_rust_files(&path, files);
            } else if path.extension().and_then(|ext| ext.to_str()) == Some("rs") {
                files.push(path);
            }
        }
    }

    pub fn scan_phase4_priority_ui_files(&self) -> Vec<ForbiddenShortcutViolation> {
        let mut violations = Vec::new();
        for relative in [
            "6.apps/editor/stratumx_editor_app/src/desktop_app/editor_app.rs",
            "6.apps/editor/stratumx_editor_app/src/desktop_app/top_bar.rs",
            "6.apps/editor/stratumx_editor_app/src/desktop_app/open_world_dialog.rs",
            "6.apps/editor/stratumx_editor_app/src/desktop_app/terrain_panel.rs",
            "6.apps/editor/stratumx_editor_app/src/desktop_app/material_panel.rs",
            "6.apps/editor/stratumx_editor_app/src/desktop_app/sky_panel.rs",
            "6.apps/editor/stratumx_editor_app/src/desktop_app/audio_panel.rs",
            "6.apps/editor/stratumx_editor_app/src/desktop_app/build_panel.rs",
            "6.apps/editor/stratumx_editor_app/src/desktop_app/viewport_panel.rs",
            "6.apps/editor/stratumx_editor_app/src/desktop_app/inspector_panel.rs",
            "6.apps/editor/stratumx_editor_app/src/desktop_app/update_loop.rs",
            "6.apps/editor/stratumx_editor_app/src/desktop_app/app_state.rs",
        ] {
            let path = self.repo_root.join(relative);
            if path.exists() {
                violations.extend(self.scan_file(&path));
            }
        }
        violations
    }

    fn scan_file(&self, path: &Path) -> Vec<ForbiddenShortcutViolation> {
        let content = match fs::read_to_string(path) {
            Ok(content) => content,
            Err(_) => return Vec::new(),
        };
        let mut violations = Vec::new();

        for (index, line) in content.lines().enumerate() {
            let trimmed = line.trim();

            if trimmed.contains("self.host.") {
                let method = trimmed
                    .split("self.host.")
                    .nth(1)
                    .unwrap_or_default()
                    .split('(')
                    .next()
                    .unwrap_or_default()
                    .trim()
                    .to_string();
                let shortcut = if is_read_only_host_query(&method) {
                    ForbiddenShortcut::ReadOnlyHostQuery { method }
                } else {
                    ForbiddenShortcut::MutatingHostCall { method }
                };
                violations.push(ForbiddenShortcutViolation {
                    file: path.to_path_buf(),
                    line: index + 1,
                    shortcut,
                    snippet: trimmed.to_string(),
                });
            }

            if trimmed.contains(".execute_command(") || trimmed.contains("fn execute_command(") {
                violations.push(ForbiddenShortcutViolation {
                    file: path.to_path_buf(),
                    line: index + 1,
                    shortcut: ForbiddenShortcut::ExecuteBridge,
                    snippet: trimmed.to_string(),
                });
            }

            if trimmed.contains("get_world_state_mut(") {
                violations.push(ForbiddenShortcutViolation {
                    file: path.to_path_buf(),
                    line: index + 1,
                    shortcut: ForbiddenShortcut::DirectWorldStateMutation,
                    snippet: trimmed.to_string(),
                });
            }

            if trimmed.contains("vertical_slice_scene_mut(")
                || trimmed.contains("world_state_mut(")
                || trimmed.contains("sdk_mutation(")
            {
                violations.push(ForbiddenShortcutViolation {
                    file: path.to_path_buf(),
                    line: index + 1,
                    shortcut: ForbiddenShortcut::DirectSdkEngineMutation,
                    snippet: trimmed.to_string(),
                });
            }
        }

        violations
    }
}

fn is_read_only_host_query(method: &str) -> bool {
    matches!(
        method,
        "get_world_state"
            | "get_diagnostics"
            | "get_viewport_stats"
            | "terrain_needs_gpu_sync"
            | "update"
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_only_host_queries_are_distinguished() {
        assert!(is_read_only_host_query("get_world_state"));
        assert!(!is_read_only_host_query("save_world"));
    }

    #[test]
    fn detects_editor_importing_engine_types() {
        let scanner = ForbiddenShortcutScanner::new(PathBuf::from("/fake/repo"));

        let line = "use l0_world_truth::WorldState;";
        assert!(scanner.is_engine_import(line));

        let line = "use crate::l1::runtime::Runtime;";
        assert!(scanner.is_engine_import(line));

        let line = "use l5_link_ingress_packets::Command;";
        assert!(!scanner.is_engine_import(line));
    }

    #[test]
    fn detects_tooling_importing_editor_types() {
        let scanner = ForbiddenShortcutScanner::new(PathBuf::from("/fake/repo"));

        let line = "use l7_editor_command_spine::EditorCommand;";
        assert!(scanner.is_editor_import(line));

        let line = "use l8_editor_shell::Shell;";
        assert!(scanner.is_editor_import(line));

        let line = "use l6_authority_core::Authority;";
        assert!(!scanner.is_editor_import(line));
    }

    #[test]
    fn detects_domain_logic_in_apps() {
        let scanner = ForbiddenShortcutScanner::new(PathBuf::from("/fake/repo"));

        // Should detect validation functions (not input validation)
        assert!(scanner.is_domain_logic_pattern("fn validate_world_state() {"));

        // Should detect calculation functions (not layout)
        assert!(scanner.is_domain_logic_pattern("fn calculate_terrain_height() {"));

        // Should detect business logic processors
        assert!(scanner.is_domain_logic_pattern("fn process_command mut data() {"));

        // Should NOT detect simple bootstrap code
        assert!(!scanner.is_domain_logic_pattern("fn main() {"));
        assert!(!scanner.is_domain_logic_pattern("// This is a comment about validation"));

        // Should NOT detect UI event handlers
        assert!(!scanner.is_domain_logic_pattern("fn handle_shortcuts(&mut self) {"));
        assert!(!scanner.is_domain_logic_pattern("fn handle_input(&mut self) {"));
    }

    #[test]
    fn extracts_import_paths_correctly() {
        let scanner = ForbiddenShortcutScanner::new(PathBuf::from("/fake/repo"));

        let path = scanner.extract_import_path("use l0_world_truth::WorldState;");
        assert_eq!(path, "l0_world_truth::WorldState");

        let path = scanner.extract_import_path("use crate::l1::runtime::Runtime;");
        assert_eq!(path, "crate::l1::runtime");
    }
}
