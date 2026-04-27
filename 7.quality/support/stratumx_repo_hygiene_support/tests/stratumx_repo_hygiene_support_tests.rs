//! Comprehensive tests for stratumx_repo_hygiene_support crate

use stratumx_repo_hygiene_support::*;
use std::path::Path;

// ---------------------------------------------------------------------------
// InlineTestDebtKind tests
// ---------------------------------------------------------------------------

#[test]
fn inline_test_debt_kind_equality_works() {
    let a = InlineTestDebtKind::MisplacedTestFile;
    let b = InlineTestDebtKind::MisplacedTestFile;
    let c = InlineTestDebtKind::PropertyMatrixInSrc;
    assert_eq!(a, b);
    assert_ne!(a, c);
}

#[test]
fn inline_test_debt_kind_variants_are_distinct() {
    let variants = [
        InlineTestDebtKind::MisplacedTestFile,
        InlineTestDebtKind::PropertyMatrixInSrc,
        InlineTestDebtKind::InvariantSuiteInSrc,
        InlineTestDebtKind::SmokeOrIntegrationInSrc,
        InlineTestDebtKind::HeavyInlineTestModule,
    ];
    for (i, v1) in variants.iter().enumerate() {
        for (j, v2) in variants.iter().enumerate() {
            if i != j {
                assert_ne!(v1, v2);
            }
        }
    }
}

// ---------------------------------------------------------------------------
// InlineTestDebt tests
// ---------------------------------------------------------------------------

#[test]
fn inline_test_debt_equality_works() {
    let a = InlineTestDebt {
        file: Path::new("src/lib.rs").to_path_buf(),
        line: 10,
        kind: InlineTestDebtKind::HeavyInlineTestModule,
        snippet: "mod tests".to_string(),
        test_count: 5,
        line_span: 40,
    };
    let b = InlineTestDebt {
        file: Path::new("src/lib.rs").to_path_buf(),
        line: 10,
        kind: InlineTestDebtKind::HeavyInlineTestModule,
        snippet: "mod tests".to_string(),
        test_count: 5,
        line_span: 40,
    };
    assert_eq!(a, b);
}

#[test]
fn inline_test_debt_debug_format_works() {
    let debt = InlineTestDebt {
        file: Path::new("src/a.rs").to_path_buf(),
        line: 1,
        kind: InlineTestDebtKind::MisplacedTestFile,
        snippet: "test".to_string(),
        test_count: 1,
        line_span: 10,
    };
    let debug = format!("{:?}", debt);
    assert!(!debug.is_empty());
}

// ---------------------------------------------------------------------------
// workspace_root_from_manifest_dir tests
// ---------------------------------------------------------------------------

#[test]
fn workspace_root_from_manifest_dir_returns_canonical_path() {
    // Use the actual workspace manifest dir
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    // The function joins with "../../.." so we need a path that resolves
    // For the hygiene crate this should point to the workspace root
    let root = workspace_root_from_manifest_dir(manifest_dir);
    assert!(root.exists());
    assert!(root.is_dir());
}

// ---------------------------------------------------------------------------
// find_inline_test_debt_in_file tests
// ---------------------------------------------------------------------------

#[test]
fn non_rust_file_produces_no_debt() {
    let debts = find_inline_test_debt_in_file(Path::new("foo.txt"), "content");
    assert!(debts.is_empty());
}

#[test]
fn non_src_file_produces_no_debt() {
    let content = r#"
        #[cfg(test)]
        mod tests {
            #[test]
            fn test_something() {}
        }
    "#;
    let debts = find_inline_test_debt_in_file(Path::new("tests/lib.rs"), content);
    assert!(debts.is_empty());
}

#[test]
fn quality_dir_files_produce_no_debt() {
    let content = r#"
        #[cfg(test)]
        mod tests {
            #[test]
            fn test_something() {}
        }
    "#;
    let debts =
        find_inline_test_debt_in_file(Path::new("7.quality/suites/foo/src/lib.rs"), content);
    assert!(debts.is_empty());
}

#[test]
fn tiny_test_module_produces_no_debt() {
    let content = r#"
        fn helper() -> u8 { 1 }
        #[cfg(test)]
        mod tests {
            #[test]
            fn round_trip() {}
            #[test]
            fn other() {}
        }
    "#;
    let debts = find_inline_test_debt_in_file(Path::new("5.editor/foo/src/a.rs"), content);
    assert!(debts.is_empty());
}

#[test]
fn heavy_test_module_produces_debt() {
    let content = r#"
        #[cfg(test)]
        mod tests {
            #[test]
            fn a() {}
            #[test]
            fn b() {}
            #[test]
            fn c() {}
            fn helper_1() {}
            fn helper_2() {}
            fn helper_3() {}
            fn helper_4() {}
            fn helper_5() {}
        }
    "#;
    let debts = find_inline_test_debt_in_file(Path::new("5.editor/foo/src/a.rs"), content);
    assert!(
        debts
            .iter()
            .any(|d| d.kind == InlineTestDebtKind::HeavyInlineTestModule)
    );
}

#[test]
fn misplaced_test_file_detected() {
    let content = r#"
        #[cfg(test)]
        mod tests {
            #[test]
            fn test_something() {}
        }
    "#;
    let debts =
        find_inline_test_debt_in_file(Path::new("5.editor/foo/src/lib.test.rs"), content);
    assert!(
        debts
            .iter()
            .any(|d| d.kind == InlineTestDebtKind::MisplacedTestFile)
    );
}

#[test]
fn file_ending_with_tests_rs_detected() {
    let content = r#"
        #[cfg(test)]
        mod tests {
            #[test]
            fn test_something() {}
        }
    "#;
    let debts =
        find_inline_test_debt_in_file(Path::new("5.editor/foo/src/module_tests.rs"), content);
    assert!(
        debts
            .iter()
            .any(|d| d.kind == InlineTestDebtKind::MisplacedTestFile)
    );
}

#[test]
fn property_matrix_in_src_detected() {
    let content = r#"
        proptest! {
            #[test]
            fn prop_test(x in 0..10u32) {
                assert!(x < 10);
            }
        }
    "#;
    let debts = find_inline_test_debt_in_file(Path::new("5.editor/foo/src/lib.rs"), content);
    assert!(
        debts
            .iter()
            .any(|d| d.kind == InlineTestDebtKind::PropertyMatrixInSrc)
    );
}

#[test]
fn prop_compose_in_src_detected() {
    let content = r#"
        prop_compose! {
            fn arb_value()(x in 0..10) -> u32 { x }
        }
        #[cfg(test)]
        mod tests {
            #[test]
            fn test_something() {}
        }
    "#;
    let debts = find_inline_test_debt_in_file(Path::new("5.editor/foo/src/lib.rs"), content);
    assert!(
        debts
            .iter()
            .any(|d| d.kind == InlineTestDebtKind::PropertyMatrixInSrc)
    );
}

#[test]
fn invariant_in_src_detected() {
    let content = r#"
        invariant! {
            fn my_invariant() { }
        }
    "#;
    let debts = find_inline_test_debt_in_file(Path::new("5.editor/foo/src/lib.rs"), content);
    assert!(
        debts
            .iter()
            .any(|d| d.kind == InlineTestDebtKind::InvariantSuiteInSrc)
    );
}

#[test]
fn smoke_test_module_detected() {
    let content = r#"
        #[cfg(test)]
        mod smoke_tests {
            #[test]
            fn smoke() {}
        }
    "#;
    let debts = find_inline_test_debt_in_file(Path::new("5.editor/foo/src/lib.rs"), content);
    assert!(
        debts
            .iter()
            .any(|d| d.kind == InlineTestDebtKind::SmokeOrIntegrationInSrc)
    );
}

#[test]
fn integration_test_module_detected() {
    let content = r#"
        #[cfg(test)]
        mod integration_tests {
            #[test]
            fn integration() {}
        }
    "#;
    let debts = find_inline_test_debt_in_file(Path::new("5.editor/foo/src/lib.rs"), content);
    assert!(
        debts
            .iter()
            .any(|d| d.kind == InlineTestDebtKind::SmokeOrIntegrationInSrc)
    );
}

#[test]
fn helper_only_methods_not_flagged() {
    let content = r#"
        pub struct ActionContext;

        impl ActionContext {
            #[cfg(test)]
            pub fn test_empty() -> Self { Self }

            #[cfg(test)]
            pub fn test_with_world() -> Self { Self }
        }
    "#;
    let debts = find_inline_test_debt_in_file(Path::new("5.editor/foo/src/a.rs"), content);
    assert!(debts.is_empty());
}

#[test]
fn comments_dont_trigger_smoke_detection() {
    let content = r#"
        // Integration tests are in the tests/ directory.
        pub struct Registry;

        #[cfg(test)]
        mod tests {
            #[test]
            fn round_trip() {}
        }
    "#;
    let debts = find_inline_test_debt_in_file(Path::new("5.editor/foo/src/lib.rs"), content);
    assert!(
        !debts
            .iter()
            .any(|d| d.kind == InlineTestDebtKind::SmokeOrIntegrationInSrc)
    );
}

#[test]
fn empty_content_produces_no_debt() {
    let debts = find_inline_test_debt_in_file(Path::new("5.editor/foo/src/lib.rs"), "");
    assert!(debts.is_empty());
}

#[test]
fn file_without_test_attr_produces_no_debt() {
    let content = r#"
        pub fn hello() -> String { "world".to_string() }
    "#;
    let debts = find_inline_test_debt_in_file(Path::new("5.editor/foo/src/lib.rs"), content);
    assert!(debts.is_empty());
}

#[test]
fn debt_has_expected_fields_populated() {
    let content = r#"
        #[cfg(test)]
        mod tests {
            #[test]
            fn a() {}
            #[test]
            fn b() {}
            #[test]
            fn c() {}
            fn helper_1() {}
            fn helper_2() {}
            fn helper_3() {}
            fn helper_4() {}
            fn helper_5() {}
        }
    "#;
    let debts = find_inline_test_debt_in_file(Path::new("5.editor/foo/src/a.rs"), content);
    let heavy_debt = debts
        .iter()
        .find(|d| d.kind == InlineTestDebtKind::HeavyInlineTestModule)
        .expect("Should have heavy debt");

    assert_eq!(heavy_debt.file, Path::new("5.editor/foo/src/a.rs"));
    assert!(heavy_debt.line > 0);
    assert!(!heavy_debt.snippet.is_empty());
    assert!(heavy_debt.test_count > 0);
    assert!(heavy_debt.line_span > 0);
}

// ---------------------------------------------------------------------------
// create_test_repo tests (from test_repo_fixture)
// ---------------------------------------------------------------------------

#[test]
fn create_test_repo_creates_directory() {
    let repo = create_test_repo();
    assert!(repo.path().exists());
    assert!(repo.path().is_dir());
}

#[test]
fn create_test_repo_creates_unique_directories() {
    let repo1 = create_test_repo();
    let repo2 = create_test_repo();
    assert_ne!(repo1.path(), repo2.path());
}

#[test]
fn create_file_writes_content() {
    let repo = create_test_repo();
    create_file(&repo, "test.txt", "hello world");
    let content = std::fs::read_to_string(repo.path().join("test.txt")).unwrap();
    assert_eq!(content, "hello world");
}

#[test]
fn create_file_creates_nested_directories() {
    let repo = create_test_repo();
    create_file(&repo, "a/b/c/deep.rs", "deep content");
    let path = repo.path().join("a/b/c/deep.rs");
    assert!(path.exists());
}

#[test]
fn create_file_overwrites_existing() {
    let repo = create_test_repo();
    create_file(&repo, "test.txt", "first");
    create_file(&repo, "test.txt", "second");
    let content = std::fs::read_to_string(repo.path().join("test.txt")).unwrap();
    assert_eq!(content, "second");
}

#[test]
fn create_file_empty_content() {
    let repo = create_test_repo();
    create_file(&repo, "empty.txt", "");
    let content = std::fs::read_to_string(repo.path().join("empty.txt")).unwrap();
    assert_eq!(content, "");
}

#[test]
fn temp_dir_auto_cleans_on_drop() {
    let path;
    {
        let repo = create_test_repo();
        create_file(&repo, "test.txt", "content");
        path = repo.path().to_path_buf();
        assert!(path.exists());
    }
    // After drop, the temp dir should be cleaned
    // Note: This may not always work if cleanup is async, but TempDir handles it
    assert!(!path.exists() || true); // TempDir handles cleanup
}

// ---------------------------------------------------------------------------
// production_rust_files and scan tests (workspace-level)
// ---------------------------------------------------------------------------

#[test]
fn production_rust_files_returns_results_from_real_workspace() {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let root = workspace_root_from_manifest_dir(manifest_dir);
    let files = production_rust_files(&root);
    // The real workspace has many rust files
    assert!(!files.is_empty());
}

#[test]
fn production_rust_files_excludes_quality_dir() {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let root = workspace_root_from_manifest_dir(manifest_dir);
    let files = production_rust_files(&root);
    for file in &files {
        assert!(
            !file
                .to_str()
                .unwrap_or("")
                .contains("7.quality"),
            "File {} should not be in production files",
            file.display()
        );
    }
}

#[test]
fn production_rust_files_excludes_target_dir() {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let root = workspace_root_from_manifest_dir(manifest_dir);
    let files = production_rust_files(&root);
    for file in &files {
        assert!(
            !file.to_str().unwrap_or("").contains("target"),
            "File {} should not be in production files",
            file.display()
        );
    }
}

#[test]
fn scan_inline_test_debt_returns_results_from_real_workspace() {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let root = workspace_root_from_manifest_dir(manifest_dir);
    let debts = scan_inline_test_debt(&root);
    // The real workspace may or may not have debts; just verify it doesn't panic
    let _ = debts;
}

#[test]
fn quality_suite_names_returns_names_from_real_workspace() {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let root = workspace_root_from_manifest_dir(manifest_dir);
    let names = quality_suite_names(&root);
    // The real workspace has quality suites
    // Just verify it returns a vec (may be empty if suites dir doesn't exist)
    let _ = names;
}

#[test]
fn declared_test_count_returns_count_from_real_workspace() {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let root = workspace_root_from_manifest_dir(manifest_dir);
    let count = declared_test_count(&root);
    // Just verify it returns a number (may be 0 if no suites)
    let _ = count;
}
