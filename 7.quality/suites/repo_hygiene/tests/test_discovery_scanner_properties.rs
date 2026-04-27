// Property-based tests for Discovery Scanner
//
// These tests validate universal properties that should hold across all inputs.

use proptest::prelude::*;
use repo_hygiene::{DiscoveryScanner, WaiverRegistry};
use std::fs;
use tempfile::TempDir;

// ============================================================================
// Test Helpers
// ============================================================================

/// Create a temporary test repository
fn create_test_repo() -> TempDir {
    TempDir::new().expect("Failed to create temp dir")
}

/// Create a file in the test repository
fn create_file(repo: &TempDir, path: &str, content: &str) {
    let full_path = repo.path().join(path);
    if let Some(parent) = full_path.parent() {
        fs::create_dir_all(parent).expect("Failed to create parent directories");
    }
    fs::write(&full_path, content).expect("Failed to write file");
}

// ============================================================================
// Property Test Generators
// ============================================================================

/// Generate a valid Rust identifier
fn arb_identifier() -> impl Strategy<Value = String> {
    "[a-z][a-z0-9_]{0,20}".prop_map(|s| s.to_string())
}

/// Generate test file content with various markers
fn arb_test_content() -> impl Strategy<Value = String> {
    prop_oneof![
        // Standard unit test
        Just(
            r#"
#[test]
fn test_something() {
    assert_eq!(1, 1);
}
"#
            .to_string()
        ),
        // Property test with proptest!
        Just(
            r#"
use proptest::prelude::*;

proptest! {
    #[test]
    fn test_property(x in 0..100) {
        assert!(x < 100);
    }
}
"#
            .to_string()
        ),
        // Property test with prop_compose!
        Just(
            r#"
use proptest::prelude::*;

prop_compose! {
    fn arb_data()(x in 0..100) -> Data {
        Data { x }
    }
}
"#
            .to_string()
        ),
        // Invariant test
        Just(
            r#"
invariant! {
    fn check_invariant(state: &State) {
        assert!(state.valid);
    }
}
"#
            .to_string()
        ),
    ]
}

/// Generate non-test file content
fn arb_non_test_content() -> impl Strategy<Value = String> {
    prop_oneof![
        // Regular function
        Just(
            r#"
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
"#
            .to_string()
        ),
        // Struct definition
        Just(
            r#"
pub struct Data {
    pub value: i32,
}
"#
            .to_string()
        ),
        // Impl block
        Just(
            r#"
impl Data {
    pub fn new(value: i32) -> Self {
        Self { value }
    }
}
"#
            .to_string()
        ),
    ]
}

/// Generate a test file name
fn arb_test_filename() -> impl Strategy<Value = String> {
    prop_oneof![
        arb_identifier().prop_map(|name| format!("{}.test.rs", name)),
        arb_identifier().prop_map(|name| format!("{}_tests.rs", name)),
        Just("smoke_test.rs".to_string()),
    ]
}

/// Generate a non-test file name
fn arb_non_test_filename() -> impl Strategy<Value = String> {
    arb_identifier().prop_map(|name| format!("{}.rs", name))
}

// ============================================================================
// Property 1: Test File Detection Completeness
// ============================================================================

// Feature: repo-sanitization-phase-1, Property 1: Test File Detection Completeness
// For any production source directory containing test files (matching patterns `*.test.rs`,
// `*_tests.rs`, or containing proptest/invariant markers), the Discovery Scanner should
// identify all such files without false negatives.
// Validates: Requirements 1.1, 1.2, 1.3, 1.4, 1.5

proptest! {
    #![proptest_config(ProptestConfig::with_cases(100))]

    #[test]
    fn prop_test_file_detection_completeness(
        test_filename in arb_test_filename(),
        test_content in arb_test_content(),
    ) {
        let repo = create_test_repo();

        // Create a test file in src/
        let test_path = format!("src/{}", test_filename);
        create_file(&repo, &test_path, &test_content);

        // Scan for test files
        let scanner = DiscoveryScanner::new(repo.path().to_path_buf(), WaiverRegistry::new());
        let detected = scanner.scan_test_files();

        // Property: All test files should be detected
        prop_assert_eq!(detected.len(), 1, "Expected to detect exactly 1 test file");
        prop_assert!(
            detected[0].path.ends_with(&test_filename),
            "Detected file should match the test filename"
        );
    }
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(100))]

    #[test]
    fn prop_no_false_positives_in_test_detection(
        non_test_filename in arb_non_test_filename(),
        non_test_content in arb_non_test_content(),
    ) {
        let repo = create_test_repo();

        // Create a non-test file in src/
        let file_path = format!("src/{}", non_test_filename);
        create_file(&repo, &file_path, &non_test_content);

        // Scan for test files
        let scanner = DiscoveryScanner::new(repo.path().to_path_buf(), WaiverRegistry::new());
        let detected = scanner.scan_test_files();

        // Property: Non-test files should not be detected as test files
        prop_assert_eq!(detected.len(), 0, "Non-test files should not be detected");
    }
}

// ============================================================================
// Property 13: Line Limit Enforcement
// ============================================================================

// Feature: repo-sanitization-phase-1, Property 13: Line Limit Enforcement
// For any production code file exceeding 200 lines that is not in the Waiver_Registry,
// the Hygiene_Checker should report a line limit violation.
// Validates: Requirements 6.1

proptest! {
    #![proptest_config(ProptestConfig::with_cases(100))]

    #[test]
    fn prop_line_limit_enforcement(
        line_count in 201usize..500usize,
        filename in arb_non_test_filename(),
    ) {
        let repo = create_test_repo();

        // Create a file with more than 200 lines
        let mut content = String::new();
        for i in 0..line_count {
            content.push_str(&format!("// Line {}\n", i));
        }

        let file_path = format!("src/{}", filename);
        create_file(&repo, &file_path, &content);

        // Scan for hygiene violations
        let scanner = DiscoveryScanner::new(repo.path().to_path_buf(), WaiverRegistry::new());
        let violations = scanner.scan_hygiene_violations();

        // Property: Files exceeding line limit should be reported
        prop_assert!(
            violations.iter().any(|v| matches!(
                v.rule,
                repo_hygiene::HygieneRule::LineLimit { limit: 200, actual } if actual == line_count
            )),
            "Expected line limit violation for file with {} lines",
            line_count
        );
    }
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(100))]

    #[test]
    fn prop_line_limit_no_false_positives(
        line_count in 1usize..200usize,
        filename in arb_non_test_filename(),
    ) {
        let repo = create_test_repo();

        // Create a file with less than 200 lines
        let mut content = String::new();
        for i in 0..line_count {
            content.push_str(&format!("// Line {}\n", i));
        }

        let file_path = format!("src/{}", filename);
        create_file(&repo, &file_path, &content);

        // Scan for hygiene violations
        let scanner = DiscoveryScanner::new(repo.path().to_path_buf(), WaiverRegistry::new());
        let violations = scanner.scan_hygiene_violations();

        // Property: Files under line limit should not be reported
        prop_assert!(
            !violations.iter().any(|v| matches!(
                v.rule,
                repo_hygiene::HygieneRule::LineLimit { .. }
            )),
            "Should not report line limit violation for file with {} lines",
            line_count
        );
    }
}

// ============================================================================
// Property 14: Test File in Src Detection
// ============================================================================

// Feature: repo-sanitization-phase-1, Property 14: Test File in Src Detection
// For any file matching test patterns (`*.test.rs`, `*_tests.rs`) in production `src/`
// directories, the Hygiene_Checker should report a violation.
// Validates: Requirements 6.2

proptest! {
    #![proptest_config(ProptestConfig::with_cases(100))]

    #[test]
    fn prop_test_file_in_src_detection(
        test_filename in arb_test_filename(),
        test_content in arb_test_content(),
    ) {
        let repo = create_test_repo();

        // Create a test file in src/
        let test_path = format!("src/{}", test_filename);
        create_file(&repo, &test_path, &test_content);

        // Scan for hygiene violations
        let scanner = DiscoveryScanner::new(repo.path().to_path_buf(), WaiverRegistry::new());
        let violations = scanner.scan_hygiene_violations();

        // Property: Test files in src/ should be reported
        prop_assert!(
            violations.iter().any(|v| matches!(
                v.rule,
                repo_hygiene::HygieneRule::TestFileInSrc
            )),
            "Expected TestFileInSrc violation for test file in src/"
        );
    }
}

// ============================================================================
// Property 15: TODO Comment Detection
// ============================================================================

// Feature: repo-sanitization-phase-1, Property 15: TODO Comment Detection
// For any production code file containing TODO comments, the Hygiene_Checker should
// report a violation with the line number and context.
// Validates: Requirements 6.3

proptest! {
    #![proptest_config(ProptestConfig::with_cases(100))]

    #[test]
    fn prop_todo_comment_detection(
        filename in arb_non_test_filename(),
        line_before in 0usize..10usize,
        line_after in 0usize..10usize,
    ) {
        let repo = create_test_repo();

        // Create a file with a TODO comment
        let mut content = String::new();
        for i in 0..line_before {
            content.push_str(&format!("// Line {}\n", i));
        }
        content.push_str("// TODO: Fix this\n");
        for i in 0..line_after {
            content.push_str(&format!("// Line {}\n", i));
        }

        let file_path = format!("src/{}", filename);
        create_file(&repo, &file_path, &content);

        // Scan for hygiene violations
        let scanner = DiscoveryScanner::new(repo.path().to_path_buf(), WaiverRegistry::new());
        let violations = scanner.scan_hygiene_violations();

        // Property: TODO comments should be detected
        prop_assert!(
            violations.iter().any(|v| matches!(
                v.rule,
                repo_hygiene::HygieneRule::TodoComment
            )),
            "Expected TODO comment violation"
        );

        // Property: Line number should be correct
        let todo_violation = violations.iter().find(|v| matches!(
            v.rule,
            repo_hygiene::HygieneRule::TodoComment
        )).unwrap();

        prop_assert_eq!(
            todo_violation.line_number,
            Some(line_before + 1),
            "TODO comment should be on line {}",
            line_before + 1
        );
    }
}

// ============================================================================
// Property 16: Allow Attribute Detection
// ============================================================================

// Feature: repo-sanitization-phase-1, Property 16: Allow Attribute Detection
// For any production code file containing `#[allow(...)]` attributes where the file is
// not in the Waiver_Registry, the Hygiene_Checker should report a violation.
// Validates: Requirements 6.4

proptest! {
    #![proptest_config(ProptestConfig::with_cases(100))]

    #[test]
    fn prop_allow_attribute_detection(
        filename in arb_non_test_filename(),
        attr_name in prop_oneof![
            Just("dead_code"),
            Just("unused_variables"),
            Just("clippy::all"),
        ],
    ) {
        let repo = create_test_repo();

        // Create a file with an #[allow(...)] attribute
        let content = format!(
            r#"
#[allow({})]
pub fn some_function() {{
    println!("Hello");
}}
"#,
            attr_name
        );

        let file_path = format!("src/{}", filename);
        create_file(&repo, &file_path, &content);

        // Scan for hygiene violations
        let scanner = DiscoveryScanner::new(repo.path().to_path_buf(), WaiverRegistry::new());
        let violations = scanner.scan_hygiene_violations();

        // Property: #[allow(...)] attributes should be detected
        prop_assert!(
            violations.iter().any(|v| matches!(
                v.rule,
                repo_hygiene::HygieneRule::AllowAttribute { .. }
            )),
            "Expected AllowAttribute violation"
        );
    }
}

// ============================================================================
// Property 6: Host Bypass Pattern Detection
// ============================================================================

// Feature: repo-sanitization-phase-1, Property 6: Host Bypass Pattern Detection
// For any Rust source file in the Desktop_App directory containing `self.host.*` method
// calls, the Desktop_App_Cleaner should identify all such patterns with their file
// location and line number.
// Validates: Requirements 2.2, 8.3

proptest! {
    #![proptest_config(ProptestConfig::with_cases(100))]

    #[test]
    fn prop_host_bypass_detection(
        method_name in prop_oneof![
            Just("save_file"),
            Just("open_file"),
            Just("read_file"),
            Just("write_file"),
        ],
    ) {
        let repo = create_test_repo();

        // Create a file with a host bypass
        let content = format!(
            r#"
impl Panel {{
    fn do_something(&self) {{
        self.host.{}();
    }}
}}
"#,
            method_name
        );

        let file_path = "6.apps/editor/stratumx_editor_app/src/desktop_app/panel.rs";
        create_file(&repo, file_path, &content);

        // Scan for host bypasses
        let scanner = DiscoveryScanner::new(repo.path().to_path_buf(), WaiverRegistry::new());
        let bypasses = scanner.scan_host_bypasses();

        // Property: Host bypasses should be detected
        prop_assert_eq!(bypasses.len(), 1, "Expected to detect 1 host bypass");
        prop_assert!(
            bypasses[0].pattern.contains(&format!("self.host.{}", method_name)),
            "Bypass pattern should contain the method call"
        );
    }
}

// ============================================================================
// Property 10: Registration Blob Size Detection
// ============================================================================

// Feature: repo-sanitization-phase-1, Property 10: Registration Blob Size Detection
// For any registration module in Command_Spine exceeding 200 lines, the
// Command_Spine_Cleaner should flag it for decomposition.
// Validates: Requirements 4.3

proptest! {
    #![proptest_config(ProptestConfig::with_cases(100))]

    #[test]
    fn prop_registration_blob_detection(
        line_count in 201usize..500usize,
    ) {
        let repo = create_test_repo();

        // Create a large registration file
        let mut content = String::from("// Registration module\n");
        for i in 0..line_count {
            content.push_str(&format!("fn register_item_{}() {{}}\n", i));
        }

        let file_path = "5.editor/l7.0-editor-command-spine/src/registration.rs";
        create_file(&repo, file_path, &content);

        // Scan for registration blobs
        let scanner = DiscoveryScanner::new(repo.path().to_path_buf(), WaiverRegistry::new());
        let blobs = scanner.scan_registration_blobs();

        // Property: Large registration modules should be detected
        prop_assert_eq!(blobs.len(), 1, "Expected to detect 1 registration blob");
        prop_assert!(
            blobs[0].line_count > 200,
            "Blob should have more than 200 lines"
        );
    }
}
