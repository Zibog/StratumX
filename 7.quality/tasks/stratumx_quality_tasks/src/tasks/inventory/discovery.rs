use super::classification::{
    count_non_blank_non_comment_lines, detect_support_code, identify_test_families,
};
use super::models::{Domain, SizeCategory, TestFileInventory};
use std::fs;
use std::path::Path;

pub fn scan_test_files(suites_dir: &Path) -> Result<Vec<TestFileInventory>, String> {
    let mut inventory = Vec::new();
    let suite_entries =
        fs::read_dir(suites_dir).map_err(|e| format!("Failed to read suites directory: {}", e))?;

    for suite_entry in suite_entries {
        let suite_entry = suite_entry.map_err(|e| e.to_string())?;
        let suite_path = suite_entry.path();
        if !suite_path.is_dir() {
            continue;
        }

        let suite_name = suite_path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("unknown")
            .to_string();

        let tests_dir = suite_path.join("tests");
        if !tests_dir.exists() {
            continue;
        }

        scan_directory_recursive(&tests_dir, &suite_name, &mut inventory)?;
    }

    Ok(inventory)
}

fn scan_directory_recursive(
    dir: &Path,
    suite_name: &str,
    inventory: &mut Vec<TestFileInventory>,
) -> Result<(), String> {
    let entries =
        fs::read_dir(dir).map_err(|e| format!("Failed to read directory {:?}: {}", dir, e))?;

    for entry in entries {
        let entry = entry.map_err(|e| e.to_string())?;
        let path = entry.path();

        if path.is_dir() {
            scan_directory_recursive(&path, suite_name, inventory)?;
        } else if path.extension().and_then(|e| e.to_str()) == Some("rs") {
            if let Ok(file_inventory) = analyze_test_file(&path, suite_name) {
                inventory.push(file_inventory);
            }
        }
    }

    Ok(())
}

fn analyze_test_file(path: &Path, suite_name: &str) -> Result<TestFileInventory, String> {
    let content =
        fs::read_to_string(path).map_err(|e| format!("Failed to read file {:?}: {}", path, e))?;

    let line_count = count_non_blank_non_comment_lines(&content);
    let size_category = SizeCategory::from_line_count(line_count);
    let (test_count, families) = identify_test_families(&content);
    let embedded_support = detect_support_code(&content);
    let domain = Domain::from_suite_name(suite_name);

    Ok(TestFileInventory {
        path: path.to_path_buf(),
        suite: suite_name.to_string(),
        line_count,
        test_count,
        families,
        embedded_support,
        size_category,
        domain,
    })
}
