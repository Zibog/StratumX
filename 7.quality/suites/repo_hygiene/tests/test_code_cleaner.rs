// Unit tests for CodeCleaner module
//
// Tests cover:
// - Host bypass pattern detection (Task 7.3)
// - Registration blob identification (Task 7.5)
// - Domain logic detection in UI code (Task 7.7)
// - Cleanup report generation (Task 7.10)

use repo_hygiene::code_cleaner::CodeCleaner;
use repo_hygiene::models::DomainLogicType;
use std::fs;
use tempfile::TempDir;

// ============================================================================
// Task 7.3: Unit tests for host bypass identification
// ============================================================================

#[test]
fn test_identify_host_bypasses_detects_save_file() {
    let temp_dir = TempDir::new().unwrap();
    let repo_root = temp_dir.path().to_path_buf();

    // Create desktop_app directory structure
    let desktop_app_dir = repo_root.join("6.apps/editor/stratumx_editor_app/src/desktop_app");
    fs::create_dir_all(&desktop_app_dir).unwrap();

    // Create a panel file with host bypass
    let panel_file = desktop_app_dir.join("project_panel.rs");
    fs::write(
        &panel_file,
        r#"
impl ProjectPanel {
    fn save_project(&self) {
        self.host.save_file(path, content);
    }
}
"#,
    )
    .unwrap();

    let cleaner = CodeCleaner::new(repo_root);
    let bypasses = cleaner.identify_host_bypasses();

    assert_eq!(bypasses.len(), 1);
    assert_eq!(bypasses[0].line, 4);
    assert!(bypasses[0].pattern.contains("self.host.save_file"));
    assert!(bypasses[0].suggested_action.contains("SaveFile"));
}

#[test]
fn test_identify_host_bypasses_detects_open_file_dialog() {
    let temp_dir = TempDir::new().unwrap();
    let repo_root = temp_dir.path().to_path_buf();

    let desktop_app_dir = repo_root.join("6.apps/editor/stratumx_editor_app/src/desktop_app");
    fs::create_dir_all(&desktop_app_dir).unwrap();

    let panel_file = desktop_app_dir.join("asset_panel.rs");
    fs::write(
        &panel_file,
        r#"
impl AssetPanel {
    fn open_asset(&self) {
        let path = self.host.open_file_dialog();
    }
}
"#,
    )
    .unwrap();

    let cleaner = CodeCleaner::new(repo_root);
    let bypasses = cleaner.identify_host_bypasses();

    assert_eq!(bypasses.len(), 1);
    assert!(bypasses[0].pattern.contains("self.host.open_file_dialog"));
    assert!(bypasses[0].suggested_action.contains("dialog"));
}

#[test]
fn test_identify_host_bypasses_multiple_patterns() {
    let temp_dir = TempDir::new().unwrap();
    let repo_root = temp_dir.path().to_path_buf();

    let desktop_app_dir = repo_root.join("6.apps/editor/stratumx_editor_app/src/desktop_app");
    fs::create_dir_all(&desktop_app_dir).unwrap();

    let panel_file = desktop_app_dir.join("multi_panel.rs");
    fs::write(
        &panel_file,
        r#"
impl MultiPanel {
    fn save(&self) {
        self.host.save_file(path, content);
    }
    
    fn open(&self) {
        self.host.open_file(path);
    }
}
"#,
    )
    .unwrap();

    let cleaner = CodeCleaner::new(repo_root);
    let bypasses = cleaner.identify_host_bypasses();

    assert_eq!(bypasses.len(), 2);
}

#[test]
fn test_identify_host_bypasses_no_bypasses() {
    let temp_dir = TempDir::new().unwrap();
    let repo_root = temp_dir.path().to_path_buf();

    let desktop_app_dir = repo_root.join("6.apps/editor/stratumx_editor_app/src/desktop_app");
    fs::create_dir_all(&desktop_app_dir).unwrap();

    let panel_file = desktop_app_dir.join("clean_panel.rs");
    fs::write(
        &panel_file,
        r#"
impl CleanPanel {
    fn save(&self) {
        // Properly routes through Command_Spine
        self.command_spine.execute(SaveFileCommand::new(path, content));
    }
}
"#,
    )
    .unwrap();

    let cleaner = CodeCleaner::new(repo_root);
    let bypasses = cleaner.identify_host_bypasses();

    assert_eq!(bypasses.len(), 0);
}

#[test]
fn test_host_bypass_remediation_suggestions() {
    let temp_dir = TempDir::new().unwrap();
    let repo_root = temp_dir.path().to_path_buf();

    let desktop_app_dir = repo_root.join("6.apps/editor/stratumx_editor_app/src/desktop_app");
    fs::create_dir_all(&desktop_app_dir).unwrap();

    // Test save_file remediation
    fs::write(
        desktop_app_dir.join("save_panel.rs"),
        "impl Panel { fn save(&self) { self.host.save_file(path, data); } }",
    )
    .unwrap();

    let cleaner = CodeCleaner::new(repo_root.clone());
    let bypasses = cleaner.identify_host_bypasses();
    assert!(!bypasses.is_empty());
    assert!(bypasses[0].suggested_action.contains("SaveFile"));
    assert!(bypasses[0].suggested_action.contains("Command_Spine"));

    // Clean up for next test
    fs::remove_file(desktop_app_dir.join("save_panel.rs")).unwrap();

    // Test read_file remediation
    fs::write(
        desktop_app_dir.join("read_panel.rs"),
        "impl Panel { fn read(&self) { self.host.read_file(path); } }",
    )
    .unwrap();

    let cleaner = CodeCleaner::new(repo_root.clone());
    let bypasses = cleaner.identify_host_bypasses();
    assert!(!bypasses.is_empty());
    assert!(bypasses[0].suggested_action.contains("OpenFile"));

    // Clean up for next test
    fs::remove_file(desktop_app_dir.join("read_panel.rs")).unwrap();

    // Test dialog remediation
    fs::write(
        desktop_app_dir.join("dialog_panel.rs"),
        "impl Panel { fn show(&self) { self.host.show_dialog(); } }",
    )
    .unwrap();

    let cleaner = CodeCleaner::new(repo_root);
    let bypasses = cleaner.identify_host_bypasses();
    assert!(!bypasses.is_empty());
    assert!(bypasses[0].suggested_action.contains("dialog"));
}

// ============================================================================
// Task 7.5: Unit tests for registration blob identification
// ============================================================================

#[test]
fn test_identify_registration_blobs_detects_large_module() {
    let temp_dir = TempDir::new().unwrap();
    let repo_root = temp_dir.path().to_path_buf();

    let command_spine_dir = repo_root.join("5.editor/l7.0-editor-command-spine/src");
    fs::create_dir_all(&command_spine_dir).unwrap();

    // Create a large registration file (>200 lines)
    let registration_file = command_spine_dir.join("command_registration.rs");
    let mut content = String::from("// Command registration module\n");
    for i in 0..250 {
        content.push_str(&format!("register_command(\"cmd_{}\", Command{});\n", i, i));
    }
    fs::write(&registration_file, content).unwrap();

    let cleaner = CodeCleaner::new(repo_root);
    let blobs = cleaner.identify_registration_blobs();

    assert_eq!(blobs.len(), 1);
    assert!(blobs[0].line_count > 200);
    assert!(blobs[0].file.ends_with("command_registration.rs"));
}

#[test]
fn test_identify_registration_blobs_detects_mixed_concerns() {
    let temp_dir = TempDir::new().unwrap();
    let repo_root = temp_dir.path().to_path_buf();

    let command_spine_dir = repo_root.join("5.editor/l7.0-editor-command-spine/src");
    fs::create_dir_all(&command_spine_dir).unwrap();

    let registration_file = command_spine_dir.join("mixed_registration.rs");
    let mut content = String::from("// Mixed registration module\n");
    // Add enough lines to exceed 200
    for i in 0..100 {
        content.push_str(&format!("register_command(\"cmd_{}\", Command{});\n", i, i));
    }
    for i in 0..100 {
        content.push_str(&format!("register_panel(\"panel_{}\", Panel{});\n", i, i));
    }
    for i in 0..50 {
        content.push_str(&format!("register_tool(\"tool_{}\", Tool{});\n", i, i));
    }
    fs::write(&registration_file, content).unwrap();

    let cleaner = CodeCleaner::new(repo_root);
    let blobs = cleaner.identify_registration_blobs();

    assert_eq!(blobs.len(), 1);
    assert!(blobs[0].mixed_concerns.len() >= 2);
    assert!(!blobs[0].suggested_decomposition.is_empty());
}

#[test]
fn test_identify_registration_blobs_ignores_small_modules() {
    let temp_dir = TempDir::new().unwrap();
    let repo_root = temp_dir.path().to_path_buf();

    let command_spine_dir = repo_root.join("5.editor/l7.0-editor-command-spine/src");
    fs::create_dir_all(&command_spine_dir).unwrap();

    let registration_file = command_spine_dir.join("small_registration.rs");
    fs::write(
        &registration_file,
        r#"
// Small registration module
pub fn register_commands() {
    register_command("save", SaveCommand);
    register_command("open", OpenCommand);
}
"#,
    )
    .unwrap();

    let cleaner = CodeCleaner::new(repo_root);
    let blobs = cleaner.identify_registration_blobs();

    assert_eq!(blobs.len(), 0);
}

#[test]
fn test_detect_mixed_concerns() {
    let temp_dir = TempDir::new().unwrap();
    let repo_root = temp_dir.path().to_path_buf();

    let command_spine_dir = repo_root.join("5.editor/l7.0-editor-command-spine/src");
    fs::create_dir_all(&command_spine_dir).unwrap();

    let registration_file = command_spine_dir.join("mixed_registration_test.rs");
    let mut content = String::new();
    for i in 0..100 {
        content.push_str(&format!("register_command(\"cmd_{}\", Cmd{});\n", i, i));
    }
    for i in 0..100 {
        content.push_str(&format!("register_panel(\"panel_{}\", Panel{});\n", i, i));
    }
    for i in 0..50 {
        content.push_str(&format!("register_tool(\"tool_{}\", Tool{});\n", i, i));
    }
    fs::write(&registration_file, content).unwrap();

    let cleaner = CodeCleaner::new(repo_root);
    let blobs = cleaner.identify_registration_blobs();

    assert!(!blobs.is_empty());
    let concerns = &blobs[0].mixed_concerns;
    assert!(concerns.iter().any(|c| c.contains("Command")));
    assert!(concerns.iter().any(|c| c.contains("Panel")));
    assert!(concerns.iter().any(|c| c.contains("Tool")));
}

#[test]
fn test_suggest_decomposition_multiple_concerns() {
    let temp_dir = TempDir::new().unwrap();
    let repo_root = temp_dir.path().to_path_buf();

    let command_spine_dir = repo_root.join("5.editor/l7.0-editor-command-spine/src");
    fs::create_dir_all(&command_spine_dir).unwrap();

    let registration_file = command_spine_dir.join("decomp_test_registration.rs");
    let mut content = String::new();
    for i in 0..100 {
        content.push_str(&format!("register_command(\"cmd_{}\", Cmd{});\n", i, i));
    }
    for i in 0..100 {
        content.push_str(&format!("register_panel(\"panel_{}\", Panel{});\n", i, i));
    }
    for i in 0..50 {
        content.push_str(&format!("register_tool(\"tool_{}\", Tool{});\n", i, i));
    }
    fs::write(&registration_file, content).unwrap();

    let cleaner = CodeCleaner::new(repo_root);
    let blobs = cleaner.identify_registration_blobs();

    assert!(!blobs.is_empty());
    let suggestions = &blobs[0].suggested_decomposition;
    assert!(!suggestions.is_empty());
    assert!(suggestions.iter().any(|s| s.contains("separate")));
}

// ============================================================================
// Task 7.7: Unit tests for domain logic identification
// ============================================================================

#[test]
fn test_identify_domain_logic_detects_parsers() {
    let temp_dir = TempDir::new().unwrap();
    let repo_root = temp_dir.path().to_path_buf();

    let desktop_app_dir = repo_root.join("6.apps/editor/stratumx_editor_app/src/desktop_app");
    fs::create_dir_all(&desktop_app_dir).unwrap();

    let panel_file = desktop_app_dir.join("parser_panel.rs");
    fs::write(
        &panel_file,
        r#"
impl ParserPanel {
    fn parse_input(&self, input: &str) -> Result<Data, Error> {
        // Parser implementation in UI code
        let data = input.parse::<Data>()?;
        Ok(data)
    }
}
"#,
    )
    .unwrap();

    let cleaner = CodeCleaner::new(repo_root);
    let violations = cleaner.identify_domain_logic_in_ui();

    assert!(!violations.is_empty());
    let parser_violations: Vec<_> = violations
        .iter()
        .filter(|v| matches!(v.violation_type, DomainLogicType::Parser))
        .collect();
    assert!(!parser_violations.is_empty());
    assert!(parser_violations[0]
        .suggested_target_layer
        .contains("domain service"));
}

#[test]
fn test_identify_domain_logic_detects_validators() {
    let temp_dir = TempDir::new().unwrap();
    let repo_root = temp_dir.path().to_path_buf();

    let desktop_app_dir = repo_root.join("6.apps/editor/stratumx_editor_app/src/desktop_app");
    fs::create_dir_all(&desktop_app_dir).unwrap();

    let panel_file = desktop_app_dir.join("validator_panel.rs");
    fs::write(
        &panel_file,
        r#"
impl ValidatorPanel {
    fn validate_project_name(&self, name: &str) -> bool {
        // Complex validation logic in UI
        name.len() > 3 && name.chars().all(|c| c.is_alphanumeric())
    }
}
"#,
    )
    .unwrap();

    let cleaner = CodeCleaner::new(repo_root);
    let violations = cleaner.identify_domain_logic_in_ui();

    let validator_violations: Vec<_> = violations
        .iter()
        .filter(|v| matches!(v.violation_type, DomainLogicType::Validator))
        .collect();
    assert!(!validator_violations.is_empty());
    assert!(validator_violations[0]
        .suggested_target_layer
        .contains("validation service"));
}

#[test]
fn test_identify_domain_logic_detects_business_rules() {
    let temp_dir = TempDir::new().unwrap();
    let repo_root = temp_dir.path().to_path_buf();

    let desktop_app_dir = repo_root.join("6.apps/editor/stratumx_editor_app/src/desktop_app");
    fs::create_dir_all(&desktop_app_dir).unwrap();

    let panel_file = desktop_app_dir.join("business_panel.rs");
    fs::write(
        &panel_file,
        r#"
impl BusinessPanel {
    fn calculate_score(&self, data: &Data) -> f64 {
        // Business rule calculation in UI
        data.value * 1.5 + data.bonus
    }
}
"#,
    )
    .unwrap();

    let cleaner = CodeCleaner::new(repo_root);
    let violations = cleaner.identify_domain_logic_in_ui();

    let business_violations: Vec<_> = violations
        .iter()
        .filter(|v| matches!(v.violation_type, DomainLogicType::BusinessRule))
        .collect();
    assert!(!business_violations.is_empty());
    assert!(business_violations[0]
        .suggested_target_layer
        .contains("business logic"));
}

#[test]
fn test_identify_domain_logic_no_violations() {
    let temp_dir = TempDir::new().unwrap();
    let repo_root = temp_dir.path().to_path_buf();

    let desktop_app_dir = repo_root.join("6.apps/editor/stratumx_editor_app/src/desktop_app");
    fs::create_dir_all(&desktop_app_dir).unwrap();

    let panel_file = desktop_app_dir.join("clean_panel.rs");
    fs::write(
        &panel_file,
        r#"
impl CleanPanel {
    fn render(&self) {
        // Pure UI code, no domain logic
        self.draw_button("Save");
    }
}
"#,
    )
    .unwrap();

    let cleaner = CodeCleaner::new(repo_root);
    let violations = cleaner.identify_domain_logic_in_ui();

    assert_eq!(violations.len(), 0);
}

// ============================================================================
// Task 7.10: Unit tests for cleanup report generation
// ============================================================================

#[test]
fn test_generate_cleanup_report_aggregates_all_issues() {
    let temp_dir = TempDir::new().unwrap();
    let repo_root = temp_dir.path().to_path_buf();

    // Create desktop_app with host bypass
    let desktop_app_dir = repo_root.join("6.apps/editor/stratumx_editor_app/src/desktop_app");
    fs::create_dir_all(&desktop_app_dir).unwrap();
    fs::write(
        desktop_app_dir.join("panel1.rs"),
        "impl Panel { fn save(&self) { self.host.save_file(p, c); } }",
    )
    .unwrap();

    // Create command_spine with registration blob
    let command_spine_dir = repo_root.join("5.editor/l7.0-editor-command-spine/src");
    fs::create_dir_all(&command_spine_dir).unwrap();
    let mut large_content = String::new();
    for i in 0..250 {
        large_content.push_str(&format!("register_command(\"cmd_{}\", Cmd{});\n", i, i));
    }
    fs::write(command_spine_dir.join("registration.rs"), large_content).unwrap();

    // Create desktop_app with domain logic
    fs::write(
        desktop_app_dir.join("panel2.rs"),
        "impl Panel { fn parse_data(&self, s: &str) -> Data { s.parse::<Data>().unwrap() } }",
    )
    .unwrap();

    let cleaner = CodeCleaner::new(repo_root);
    let report = cleaner.generate_cleanup_report();

    assert!(!report.host_bypasses.is_empty());
    assert!(!report.registration_blobs.is_empty());
    assert!(!report.domain_logic_violations.is_empty());
    assert_eq!(
        report.total_issues,
        report.host_bypasses.len()
            + report.registration_blobs.len()
            + report.domain_logic_violations.len()
    );
}

#[test]
fn test_generate_cleanup_report_empty_repo() {
    let temp_dir = TempDir::new().unwrap();
    let repo_root = temp_dir.path().to_path_buf();

    let cleaner = CodeCleaner::new(repo_root);
    let report = cleaner.generate_cleanup_report();

    assert_eq!(report.host_bypasses.len(), 0);
    assert_eq!(report.registration_blobs.len(), 0);
    assert_eq!(report.domain_logic_violations.len(), 0);
    assert_eq!(report.total_issues, 0);
}

#[test]
fn test_cleanup_report_includes_remediation_guidance() {
    let temp_dir = TempDir::new().unwrap();
    let repo_root = temp_dir.path().to_path_buf();

    let desktop_app_dir = repo_root.join("6.apps/editor/stratumx_editor_app/src/desktop_app");
    fs::create_dir_all(&desktop_app_dir).unwrap();
    fs::write(
        desktop_app_dir.join("panel.rs"),
        "impl Panel { fn save(&self) { self.host.save_file(p, c); } }",
    )
    .unwrap();

    let cleaner = CodeCleaner::new(repo_root);
    let report = cleaner.generate_cleanup_report();

    // Verify remediation guidance is included
    assert!(!report.host_bypasses.is_empty());
    assert!(!report.host_bypasses[0].suggested_action.is_empty());
    assert!(report.host_bypasses[0]
        .suggested_action
        .contains("Command_Spine"));
}
