//! Проверка: #![allow(...)] атрибуты запрещены в active path

use std::fs;
use std::path::Path;

#[test]
fn no_allow_attrs_in_active_path() {
    let workspace_root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .parent()
        .unwrap();

    let mut violations = Vec::new();

    check_directory(&workspace_root.join("2.engine"), &mut violations);
    check_directory(&workspace_root.join("3.sdk"), &mut violations);
    check_directory(&workspace_root.join("4.tooling"), &mut violations);
    check_directory(&workspace_root.join("5.editor"), &mut violations);
    check_directory(&workspace_root.join("6.apps"), &mut violations);

    if !violations.is_empty() {
        eprintln!("Allow attributes found in active paths:");
        for (path, line, attr) in &violations {
            eprintln!("  {}:{} - {}", path, line, attr);
        }
        panic!("{} allow attributes in active paths", violations.len());
    }
}

fn check_directory(dir: &Path, violations: &mut Vec<(String, usize, String)>) {
    if !dir.exists() {
        return;
    }

    for entry in fs::read_dir(dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();

        if path.is_dir() {
            check_directory(&path, violations);
        } else if path.extension().is_some_and(|e| e == "rs") {
            check_file(&path, violations);
        }
    }
}

fn check_file(path: &Path, violations: &mut Vec<(String, usize, String)>) {
    if let Ok(content) = fs::read_to_string(path) {
        for (line_num, line) in content.lines().enumerate() {
            let trimmed = line.trim();
            if trimmed.starts_with("#![allow(dead_code)]")
                || trimmed.starts_with("#![allow(unused_imports)]")
                || trimmed.starts_with("#![allow(unused_mut)]")
                || trimmed.starts_with("#![allow(unused_variables)]")
            {
                violations.push((
                    path.display().to_string(),
                    line_num + 1,
                    trimmed.to_string(),
                ));
            }
        }
    }
}
