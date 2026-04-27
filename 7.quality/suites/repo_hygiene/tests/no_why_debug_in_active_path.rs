//! Проверка: WhyDebug запрещен в активных путях
//! WhyDebug был удален из кодовой базы и не должен возвращаться

use std::fs;
use std::path::Path;

#[test]
fn no_why_debug_in_active_path() {
    let workspace_root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .parent()
        .unwrap();

    let mut violations = Vec::new();

    check_for_why_debug(&workspace_root.join("2.engine"), &mut violations);
    check_for_why_debug(&workspace_root.join("3.sdk"), &mut violations);
    check_for_why_debug(&workspace_root.join("4.tooling"), &mut violations);
    check_for_why_debug(&workspace_root.join("5.editor"), &mut violations);
    check_for_why_debug(&workspace_root.join("6.apps"), &mut violations);

    if !violations.is_empty() {
        eprintln!("WhyDebug references found in active paths:");
        for (path, line, content) in &violations {
            eprintln!("  {}:{}", path, line);
            eprintln!("    {}", content.trim());
        }
        panic!("{} WhyDebug references in active paths", violations.len());
    }
}

fn check_for_why_debug(dir: &Path, violations: &mut Vec<(String, usize, String)>) {
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
            check_for_why_debug(&path, violations);
        } else if path.extension().map_or(false, |e| e == "rs") {
            if let Ok(content) = fs::read_to_string(&path) {
                let relative_path = path
                    .strip_prefix(workspace_root)
                    .unwrap()
                    .display()
                    .to_string();

                for (line_num, line) in content.lines().enumerate() {
                    let line_lower = line.to_lowercase();

                    if line_lower.contains("why_debug") || line_lower.contains("whydebug") {
                        violations.push((relative_path.clone(), line_num + 1, line.to_string()));
                    }
                }
            }
        }
    }
}
