//! Проверка: legacy директории запрещены в активных путях

use std::fs;
use std::path::Path;

#[test]
fn no_legacy_dirs_in_active_path() {
    let workspace_root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .parent()
        .unwrap();

    let mut violations = Vec::new();

    check_for_legacy_dirs(&workspace_root.join("2.engine"), &mut violations);
    check_for_legacy_dirs(&workspace_root.join("3.sdk"), &mut violations);
    check_for_legacy_dirs(&workspace_root.join("4.tooling"), &mut violations);
    check_for_legacy_dirs(&workspace_root.join("5.editor"), &mut violations);
    check_for_legacy_dirs(&workspace_root.join("6.apps"), &mut violations);

    if !violations.is_empty() {
        eprintln!("Legacy directories found in active paths:");
        for path in &violations {
            eprintln!("  {}", path);
        }
        panic!("{} legacy directories in active paths", violations.len());
    }
}

fn check_for_legacy_dirs(dir: &Path, violations: &mut Vec<String>) {
    if !dir.exists() {
        return;
    }

    for entry in fs::read_dir(dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();

        if path.is_dir() {
            let dir_name = path.file_name().unwrap().to_str().unwrap();

            if dir_name == "legacy" {
                violations.push(
                    path.strip_prefix(dir.parent().unwrap().parent().unwrap())
                        .unwrap()
                        .display()
                        .to_string(),
                );
            }

            check_for_legacy_dirs(&path, violations);
        }
    }
}
