// Property-based tests for CodeCleaner module
//
// Feature: repo-sanitization-phase-1
// Property 8: Violation Reporting Includes Remediation
//
// For any detected violation (host bypass, domain logic, registration blob),
// the generated report should include remediation guidance describing the
// correct approach.
//
// Validates: Requirements 2.4, 2.5, 3.3, 3.4, 4.5, 8.7

use proptest::prelude::*;
use repo_hygiene::code_cleaner::CodeCleaner;
use std::fs;
use tempfile::TempDir;

// ============================================================================
// Property 8: Violation Reporting Includes Remediation
// ============================================================================

// Feature: repo-sanitization-phase-1, Property 8: Violation Reporting Includes Remediation
// **Validates: Requirements 2.4, 2.5, 3.3, 3.4, 4.5, 8.7**
#[test]
fn prop_violation_reporting_includes_remediation() {
    proptest!(|(
        host_method in prop::sample::select(vec![
            "save_file", "open_file", "read_file", "write_file",
            "open_file_dialog", "save_file_dialog", "show_dialog"
        ]),
        registration_line_count in 201usize..500usize,
        domain_logic_type in prop::sample::select(vec![
            "parse_", "validate_", "calculate_", "compute_", "apply_rule"
        ])
    )| {
        let temp_dir = TempDir::new().unwrap();
        let repo_root = temp_dir.path().to_path_buf();

        // Create desktop_app with host bypass
        let desktop_app_dir = repo_root.join("6.apps/editor/stratumx_editor_app/src/desktop_app");
        fs::create_dir_all(&desktop_app_dir).unwrap();

        let host_bypass_content = format!(
            "impl Panel {{ fn action(&self) {{ self.host.{}(path); }} }}",
            host_method
        );
        fs::write(desktop_app_dir.join("host_panel.rs"), host_bypass_content).unwrap();

        // Create command_spine with registration blob
        let command_spine_dir = repo_root.join("5.editor/l7.0-editor-command-spine/src");
        fs::create_dir_all(&command_spine_dir).unwrap();

        let mut registration_content = String::new();
        for i in 0..registration_line_count {
            registration_content.push_str(&format!("register_command(\"cmd_{}\", Cmd{});\n", i, i));
        }
        fs::write(command_spine_dir.join("registration.rs"), registration_content).unwrap();

        // Create desktop_app with domain logic
        let domain_logic_content = format!(
            "impl Panel {{ fn {}data(&self, s: &str) -> Data {{ Data::new() }} }}",
            domain_logic_type
        );
        fs::write(desktop_app_dir.join("domain_panel.rs"), domain_logic_content).unwrap();

        let cleaner = CodeCleaner::new(repo_root);
        let report = cleaner.generate_cleanup_report();

        // Property: All host bypasses must have non-empty remediation guidance
        for bypass in &report.host_bypasses {
            prop_assert!(!bypass.suggested_action.is_empty(),
                "Host bypass at {}:{} has empty remediation",
                bypass.file.display(), bypass.line);
            prop_assert!(bypass.suggested_action.contains("Command_Spine"),
                "Host bypass remediation should mention Command_Spine: {}",
                bypass.suggested_action);
        }

        // Property: All registration blobs must have non-empty decomposition suggestions
        for blob in &report.registration_blobs {
            prop_assert!(!blob.suggested_decomposition.is_empty(),
                "Registration blob at {} has empty decomposition suggestions",
                blob.file.display());
            prop_assert!(!blob.suggested_decomposition.is_empty(),
                "Registration blob should have at least one decomposition suggestion");
        }

        // Property: All domain logic violations must have non-empty target layer suggestions
        for violation in &report.domain_logic_violations {
            prop_assert!(!violation.suggested_target_layer.is_empty(),
                "Domain logic violation at {} has empty target layer suggestion",
                violation.file.display());
            prop_assert!(
                violation.suggested_target_layer.contains("layer") ||
                violation.suggested_target_layer.contains("service"),
                "Domain logic remediation should mention target layer or service: {}",
                violation.suggested_target_layer
            );
        }
    });
}

// Additional property test: Remediation guidance should be specific to violation type
#[test]
fn prop_remediation_guidance_is_specific() {
    proptest!(|(
        method_type in prop::sample::select(vec!["save", "open", "dialog"])
    )| {
        let temp_dir = TempDir::new().unwrap();
        let repo_root = temp_dir.path().to_path_buf();

        let desktop_app_dir = repo_root.join("6.apps/editor/stratumx_editor_app/src/desktop_app");
        fs::create_dir_all(&desktop_app_dir).unwrap();

        let method_name = match method_type {
            "save" => "save_file",
            "open" => "open_file",
            "dialog" => "open_file_dialog",
            _ => "unknown",
        };

        let content = format!(
            "impl Panel {{ fn action(&self) {{ self.host.{}(path); }} }}",
            method_name
        );
        fs::write(desktop_app_dir.join("panel.rs"), content).unwrap();

        let cleaner = CodeCleaner::new(repo_root);
        let bypasses = cleaner.identify_host_bypasses();

        prop_assert!(!bypasses.is_empty(), "Should detect host bypass");

        // Property: Remediation should be specific to the method type
        let suggestion = &bypasses[0].suggested_action;
        match method_type {
            "save" => prop_assert!(suggestion.contains("SaveFile"),
                "Save operations should suggest SaveFile action"),
            "open" => prop_assert!(suggestion.contains("OpenFile") || suggestion.contains("dialog"),
                "Open operations should suggest OpenFile or dialog action"),
            "dialog" => prop_assert!(suggestion.contains("dialog"),
                "Dialog operations should suggest dialog action"),
            _ => {}
        }
    });
}

// Property test: Registration blob detection should identify mixed concerns
#[test]
fn prop_registration_blob_identifies_mixed_concerns() {
    proptest!(|(
        command_count in 50usize..150usize,
        panel_count in 50usize..150usize,
        include_tools in prop::bool::ANY
    )| {
        let temp_dir = TempDir::new().unwrap();
        let repo_root = temp_dir.path().to_path_buf();

        let command_spine_dir = repo_root.join("5.editor/l7.0-editor-command-spine/src");
        fs::create_dir_all(&command_spine_dir).unwrap();

        let mut content = String::new();
        for i in 0..command_count {
            content.push_str(&format!("register_command(\"cmd_{}\", Cmd{});\n", i, i));
        }
        for i in 0..panel_count {
            content.push_str(&format!("register_panel(\"panel_{}\", Panel{});\n", i, i));
        }
        if include_tools {
            for i in 0..50 {
                content.push_str(&format!("register_tool(\"tool_{}\", Tool{});\n", i, i));
            }
        }

        fs::write(command_spine_dir.join("registration.rs"), content).unwrap();

        let cleaner = CodeCleaner::new(repo_root);
        let blobs = cleaner.identify_registration_blobs();

        if !blobs.is_empty() {
            let blob = &blobs[0];

            // Property: If we have multiple registration types, mixed concerns should be detected
            let expected_concerns = if include_tools { 3 } else { 2 };
            prop_assert!(blob.mixed_concerns.len() >= expected_concerns - 1,
                "Should detect at least {} mixed concerns, found {}",
                expected_concerns - 1, blob.mixed_concerns.len());

            // Property: Decomposition suggestions should be provided for mixed concerns
            if blob.mixed_concerns.len() > 1 {
                prop_assert!(!blob.suggested_decomposition.is_empty(),
                    "Should provide decomposition suggestions for mixed concerns");
            }
        }
    });
}

// Property test: Domain logic detection should identify all violation types
#[test]
fn prop_domain_logic_detection_comprehensive() {
    proptest!(|(
        logic_type in prop::sample::select(vec!["parser", "validator", "business_rule"])
    )| {
        let temp_dir = TempDir::new().unwrap();
        let repo_root = temp_dir.path().to_path_buf();

        let desktop_app_dir = repo_root.join("6.apps/editor/stratumx_editor_app/src/desktop_app");
        fs::create_dir_all(&desktop_app_dir).unwrap();

        let content = match logic_type {
            "parser" => "impl Panel { fn parse_input(&self, s: &str) -> Data { s.parse::<Data>().unwrap() } }",
            "validator" => "impl Panel { fn validate_name(&self, s: &str) -> bool { s.len() > 3 } }",
            "business_rule" => "impl Panel { fn calculate_score(&self, d: &Data) -> f64 { d.value * 1.5 } }",
            _ => "impl Panel { fn render(&self) {} }",
        };

        fs::write(desktop_app_dir.join("panel.rs"), content).unwrap();

        let cleaner = CodeCleaner::new(repo_root);
        let violations = cleaner.identify_domain_logic_in_ui();

        if logic_type != "render" {
            prop_assert!(!violations.is_empty(),
                "Should detect {} violation", logic_type);

            // Property: Each violation should have appropriate target layer suggestion
            for violation in &violations {
                prop_assert!(!violation.suggested_target_layer.is_empty(),
                    "Violation should have target layer suggestion");

                match logic_type {
                    "parser" => prop_assert!(
                        violation.suggested_target_layer.contains("domain") ||
                        violation.suggested_target_layer.contains("service"),
                        "Parser should suggest domain/service layer"
                    ),
                    "validator" => prop_assert!(
                        violation.suggested_target_layer.contains("validation") ||
                        violation.suggested_target_layer.contains("service"),
                        "Validator should suggest validation/service layer"
                    ),
                    "business_rule" => prop_assert!(
                        violation.suggested_target_layer.contains("business") ||
                        violation.suggested_target_layer.contains("logic"),
                        "Business rule should suggest business logic layer"
                    ),
                    _ => {}
                }
            }
        }
    });
}
