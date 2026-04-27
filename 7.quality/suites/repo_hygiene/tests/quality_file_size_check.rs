//! File size check for 7.quality test contour
//!
//! This test enforces the File Size Law documented in 7.quality/docs/FILE_SIZE_LAW.md
//!
//! Test file thresholds:
//! - 0-300 lines: Normal, acceptable
//! - 301-500 lines: Suspicious, review recommended
//! - 501-800 lines: Requires split plan
//! - 800+ lines: Mandatory split (FAILS test)
//!
//! Support file thresholds:
//! - 0-250 lines: Normal, acceptable
//! - 251-400 lines: Requires split plan
//! - 400+ lines: Mandatory split (FAILS test)

use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
struct FileViolation {
    path: PathBuf,
    line_count: usize,
    category: ViolationCategory,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum ViolationCategory {
    TestSuspicious,        // 301-500 lines
    TestRequiresSplit,     // 501-800 lines
    TestMandatorySplit,    // 800+ lines
    SupportRequiresSplit,  // 251-400 lines
    SupportMandatorySplit, // 400+ lines
}

impl ViolationCategory {
    fn is_mandatory(&self) -> bool {
        matches!(
            self,
            ViolationCategory::TestMandatorySplit | ViolationCategory::SupportMandatorySplit
        )
    }
}

#[test]
fn quality_file_size_check() {
    let workspace_root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .parent()
        .unwrap();

    let quality_root = workspace_root.join("7.quality");

    let mut violations = Vec::new();

    // Check test files in 7.quality/suites/*/tests/
    check_test_files(&quality_root, &mut violations);

    // Check support files in 7.quality/support/*/src/
    check_support_files(&quality_root, &mut violations);

    // Report all violations
    if !violations.is_empty() {
        report_violations(&violations);

        // Check if any violations are mandatory splits
        let mandatory_violations: Vec<_> = violations
            .iter()
            .filter(|v| v.category.is_mandatory())
            .collect();

        if !mandatory_violations.is_empty() {
            panic!(
                "{} files exceed mandatory split threshold and MUST be split immediately",
                mandatory_violations.len()
            );
        }
    }
}

fn check_test_files(quality_root: &Path, violations: &mut Vec<FileViolation>) {
    let suites_dir = quality_root.join("suites");
    if !suites_dir.exists() {
        return;
    }

    // Iterate through each suite directory
    for suite_entry in fs::read_dir(&suites_dir).unwrap() {
        let suite_entry = suite_entry.unwrap();
        let suite_path = suite_entry.path();

        if !suite_path.is_dir() {
            continue;
        }

        // Check the tests/ subdirectory
        let tests_dir = suite_path.join("tests");
        if tests_dir.exists() && tests_dir.is_dir() {
            scan_directory_for_rust_files(&tests_dir, quality_root, violations, true);
        }
    }
}

fn check_support_files(quality_root: &Path, violations: &mut Vec<FileViolation>) {
    let support_dir = quality_root.join("support");
    if !support_dir.exists() {
        return;
    }

    // Iterate through each support package directory
    for support_entry in fs::read_dir(&support_dir).unwrap() {
        let support_entry = support_entry.unwrap();
        let support_path = support_entry.path();

        if !support_path.is_dir() {
            continue;
        }

        // Check the src/ subdirectory
        let src_dir = support_path.join("src");
        if src_dir.exists() && src_dir.is_dir() {
            scan_directory_for_rust_files(&src_dir, quality_root, violations, false);
        }
    }
}

fn scan_directory_for_rust_files(
    dir: &Path,
    quality_root: &Path,
    violations: &mut Vec<FileViolation>,
    is_test_file: bool,
) {
    for entry in fs::read_dir(dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();

        if path.is_dir() {
            // Recursively scan subdirectories
            scan_directory_for_rust_files(&path, quality_root, violations, is_test_file);
        } else if path.extension().map_or(false, |e| e == "rs") {
            check_file_size(&path, quality_root, violations, is_test_file);
        }
    }
}

fn check_file_size(
    file_path: &Path,
    quality_root: &Path,
    violations: &mut Vec<FileViolation>,
    is_test_file: bool,
) {
    if let Ok(content) = fs::read_to_string(file_path) {
        let line_count = count_lines(&content);

        let category = if is_test_file {
            // Test file thresholds
            if line_count >= 800 {
                Some(ViolationCategory::TestMandatorySplit)
            } else if line_count >= 501 {
                Some(ViolationCategory::TestRequiresSplit)
            } else if line_count >= 301 {
                Some(ViolationCategory::TestSuspicious)
            } else {
                None
            }
        } else {
            // Support file thresholds
            if line_count >= 400 {
                Some(ViolationCategory::SupportMandatorySplit)
            } else if line_count >= 251 {
                Some(ViolationCategory::SupportRequiresSplit)
            } else {
                None
            }
        };

        if let Some(category) = category {
            let relative_path = file_path.strip_prefix(quality_root).unwrap().to_path_buf();
            violations.push(FileViolation {
                path: relative_path,
                line_count,
                category,
            });
        }
    }
}

/// Count lines according to FILE_SIZE_LAW.md rules
///
/// Lines that COUNT:
/// - Code lines
/// - Documentation comments (/// and //!)
/// - Inline comments (// and /* */)
/// - Blank lines within functions or structs
///
/// Lines that DO NOT COUNT:
/// - Blank lines between top-level items
/// - File-level module documentation (first comment block)
/// - Import statements (use declarations)
fn count_lines(content: &str) -> usize {
    let lines: Vec<&str> = content.lines().collect();
    let mut count = 0;
    let mut in_file_doc = true;
    let mut in_imports = false;
    let mut prev_line_blank = false;
    let mut brace_depth: i32 = 0;

    for line in lines {
        let trimmed = line.trim();

        // Skip file-level module docs at the start
        if in_file_doc {
            if trimmed.starts_with("//!") || trimmed.starts_with("/*!") {
                continue;
            } else if !trimmed.is_empty() {
                in_file_doc = false;
            } else {
                continue;
            }
        }

        // Skip import statements
        if trimmed.starts_with("use ") || trimmed.starts_with("pub use ") {
            in_imports = true;
            continue;
        } else if in_imports && trimmed.is_empty() {
            continue;
        } else if in_imports && !trimmed.is_empty() {
            in_imports = false;
        }

        // Track brace depth to determine if we're in top-level or inside items
        for ch in line.chars() {
            match ch {
                '{' => brace_depth += 1,
                '}' => brace_depth = brace_depth.saturating_sub(1),
                _ => {}
            }
        }

        let in_top_level = brace_depth == 0;

        // Skip blank lines between top-level items
        if trimmed.is_empty() {
            if in_top_level && prev_line_blank {
                continue;
            }
            prev_line_blank = true;
        } else {
            prev_line_blank = false;
        }

        count += 1;
    }

    count
}

fn report_violations(violations: &[FileViolation]) {
    eprintln!("\n=== FILE SIZE LAW VIOLATIONS ===\n");
    eprintln!("See 7.quality/docs/FILE_SIZE_LAW.md for details\n");

    // Group by category
    let mut mandatory_test: Vec<_> = violations
        .iter()
        .filter(|v| v.category == ViolationCategory::TestMandatorySplit)
        .collect();
    mandatory_test.sort_by_key(|v| std::cmp::Reverse(v.line_count));

    let mut requires_split_test: Vec<_> = violations
        .iter()
        .filter(|v| v.category == ViolationCategory::TestRequiresSplit)
        .collect();
    requires_split_test.sort_by_key(|v| std::cmp::Reverse(v.line_count));

    let mut suspicious_test: Vec<_> = violations
        .iter()
        .filter(|v| v.category == ViolationCategory::TestSuspicious)
        .collect();
    suspicious_test.sort_by_key(|v| std::cmp::Reverse(v.line_count));

    let mut mandatory_support: Vec<_> = violations
        .iter()
        .filter(|v| v.category == ViolationCategory::SupportMandatorySplit)
        .collect();
    mandatory_support.sort_by_key(|v| std::cmp::Reverse(v.line_count));

    let mut requires_split_support: Vec<_> = violations
        .iter()
        .filter(|v| v.category == ViolationCategory::SupportRequiresSplit)
        .collect();
    requires_split_support.sort_by_key(|v| std::cmp::Reverse(v.line_count));

    // Report mandatory splits first (these fail the test)
    if !mandatory_test.is_empty() {
        eprintln!("TEST FILES - MANDATORY SPLIT (800+ lines) - BLOCKS MERGE:");
        for v in &mandatory_test {
            eprintln!("  {} ({} lines)", v.path.display(), v.line_count);
        }
        eprintln!();
    }

    if !mandatory_support.is_empty() {
        eprintln!("SUPPORT FILES - MANDATORY SPLIT (400+ lines) - BLOCKS MERGE:");
        for v in &mandatory_support {
            eprintln!("  {} ({} lines)", v.path.display(), v.line_count);
        }
        eprintln!();
    }

    // Report requires split (warnings)
    if !requires_split_test.is_empty() {
        eprintln!("TEST FILES - REQUIRES SPLIT PLAN (501-800 lines):");
        for v in &requires_split_test {
            eprintln!("  {} ({} lines)", v.path.display(), v.line_count);
        }
        eprintln!();
    }

    if !requires_split_support.is_empty() {
        eprintln!("SUPPORT FILES - REQUIRES SPLIT PLAN (251-400 lines):");
        for v in &requires_split_support {
            eprintln!("  {} ({} lines)", v.path.display(), v.line_count);
        }
        eprintln!();
    }

    // Report suspicious (informational)
    if !suspicious_test.is_empty() {
        eprintln!("TEST FILES - SUSPICIOUS (301-500 lines) - REVIEW RECOMMENDED:");
        for v in &suspicious_test {
            eprintln!("  {} ({} lines)", v.path.display(), v.line_count);
        }
        eprintln!();
    }

    eprintln!("=== SUMMARY ===");
    eprintln!("Total violations: {}", violations.len());
    eprintln!(
        "  Mandatory splits: {}",
        mandatory_test.len() + mandatory_support.len()
    );
    eprintln!(
        "  Requires split plan: {}",
        requires_split_test.len() + requires_split_support.len()
    );
    eprintln!("  Suspicious: {}", suspicious_test.len());
    eprintln!();
}
