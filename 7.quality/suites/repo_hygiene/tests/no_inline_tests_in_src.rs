// Repository Hygiene: No Heavy Inline Tests in Production Code
//
// Tiny local tests are allowed in src when they stay truly local.
// Heavy matrices, property tests, smoke tests, or integration-style tests
// must live in 7.quality/suites/* or crate-level tests/ directories.

use repo_hygiene::{HygieneRule, WaiverRegistry};
use stratumx_repo_hygiene_support::{scan_inline_test_debt, workspace_root_from_manifest_dir};

#[test]
fn no_heavy_inline_tests_in_production_code() {
    let workspace_root = workspace_root_from_manifest_dir(env!("CARGO_MANIFEST_DIR"));
    let waiver_path = workspace_root.join("7.quality/suites/repo_hygiene/waivers.toml");
    let waiver_registry =
        WaiverRegistry::load_from_file(&waiver_path).expect("waiver registry should load");
    let test_in_src_rule = HygieneRule::TestFileInSrc;
    let violations = scan_inline_test_debt(&workspace_root)
        .into_iter()
        .filter(|violation| {
            let relative_path = violation
                .file
                .strip_prefix(&workspace_root)
                .unwrap_or(violation.file.as_path());
            !waiver_registry.is_waived(&test_in_src_rule, relative_path)
        })
        .collect::<Vec<_>>();

    if !violations.is_empty() {
        let mut error_msg = String::from("\nFound heavy inline test debt in production code:\n\n");
        for violation in &violations {
            error_msg.push_str(&format!(
                "  {}:{} {:?}\n    {}\n\n",
                violation.file.display(),
                violation.line,
                violation.kind,
                violation.snippet.trim()
            ));
        }
        error_msg.push_str(
            "Move heavy, property, smoke, or integration-style tests to 7.quality/suites/* or crate-level tests/.\n",
        );
        panic!("{}", error_msg);
    }
}
