//! Проверка: в самом suite `repo_hygiene` не должно быть placeholder assertions.
//! Остальной quality-backlog отслеживается отдельными roadmap-проходами и waiver-aware hygiene checks.

use std::fs;
use std::path::Path;

#[test]
fn no_placeholder_assertions_in_quality() {
    let workspace_root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .parent()
        .unwrap();

    let mut violations = Vec::new();

    check_for_placeholder_assertions(
        &workspace_root.join("7.quality/suites/repo_hygiene"),
        &mut violations,
    );

    if !violations.is_empty() {
        eprintln!("Placeholder assertions found in quality suites:");
        for (path, line, content) in &violations {
            eprintln!("  {}:{}", path, line);
            eprintln!("    {}", content.trim());
        }
        eprintln!("\nReplace assert!(true, ...) with assertions that exercise real repo_hygiene behavior.");
        panic!(
            "{} placeholder assertions in quality suites",
            violations.len()
        );
    }
}

fn check_for_placeholder_assertions(dir: &Path, violations: &mut Vec<(String, usize, String)>) {
    if !dir.exists() {
        return;
    }

    let workspace_root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .parent()
        .unwrap();

    for entry in fs::read_dir(dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();

        if path.is_dir() {
            check_for_placeholder_assertions(&path, violations);
        } else if path.extension().map_or(false, |e| e == "rs") {
            if let Ok(content) = fs::read_to_string(&path) {
                let relative_path = path
                    .strip_prefix(workspace_root)
                    .unwrap()
                    .display()
                    .to_string();

                for (line_num, line) in content.lines().enumerate() {
                    let trimmed = line.trim();

                    if trimmed.starts_with("assert!(true")
                        || trimmed.starts_with("assert_eq!(true, true")
                    {
                        violations.push((relative_path.clone(), line_num + 1, line.to_string()));
                    }
                }
            }
        }
    }
}
