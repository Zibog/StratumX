//! Проверка: ни один файл не должен превышать 200 строк

use repo_hygiene::{HygieneRule, WaiverRegistry};
use std::fs;
use std::path::Path;

#[test]
fn no_file_over_200_lines() {
    let workspace_root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .parent()
        .unwrap();

    let waiver_path = workspace_root.join("7.quality/suites/repo_hygiene/waivers.toml");
    let waiver_registry =
        WaiverRegistry::load_from_file(&waiver_path).expect("waiver registry should load");

    let mut violations = Vec::new();

    check_directory(
        &workspace_root.join("2.engine"),
        workspace_root,
        &waiver_registry,
        &mut violations,
    );
    check_directory(
        &workspace_root.join("3.sdk"),
        workspace_root,
        &waiver_registry,
        &mut violations,
    );
    check_directory(
        &workspace_root.join("4.tooling"),
        workspace_root,
        &waiver_registry,
        &mut violations,
    );
    check_directory(
        &workspace_root.join("5.editor"),
        workspace_root,
        &waiver_registry,
        &mut violations,
    );
    check_directory(
        &workspace_root.join("6.apps"),
        workspace_root,
        &waiver_registry,
        &mut violations,
    );

    if !violations.is_empty() {
        eprintln!("Files exceeding 200 lines:");
        for (path, lines) in &violations {
            eprintln!("  {} ({} lines)", path, lines);
        }
        panic!("{} files exceed 200 lines", violations.len());
    }
}

fn check_directory(
    dir: &Path,
    workspace_root: &Path,
    waiver_registry: &WaiverRegistry,
    violations: &mut Vec<(String, usize)>,
) {
    if !dir.exists() {
        return;
    }

    for entry in fs::read_dir(dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();

        if path.is_dir() {
            check_directory(&path, workspace_root, waiver_registry, violations);
        } else if path.extension().map_or(false, |e| e == "rs") {
            if let Ok(content) = fs::read_to_string(&path) {
                let line_count = content.lines().count();
                if line_count > 200 {
                    let relative_path = path.strip_prefix(workspace_root).unwrap();
                    let rule = HygieneRule::LineLimit {
                        limit: 200,
                        actual: line_count,
                    };
                    if !waiver_registry.is_waived(&rule, relative_path) {
                        violations.push((relative_path.display().to_string(), line_count));
                    }
                }
            }
        }
    }
}
