//! Проверка: demo/debug/proof имена запрещены в release путях

use std::fs;
use std::path::Path;

#[test]
fn no_demo_debug_proof_names_in_release_path() {
    let workspace_root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .parent()
        .unwrap();

    let mut violations = Vec::new();

    check_for_forbidden_names(&workspace_root.join("2.engine"), &mut violations);
    check_for_forbidden_names(&workspace_root.join("3.sdk"), &mut violations);
    check_for_forbidden_names(&workspace_root.join("4.tooling"), &mut violations);
    check_for_forbidden_names(&workspace_root.join("5.editor"), &mut violations);

    if !violations.is_empty() {
        eprintln!("Forbidden names found in release paths:");
        for path in &violations {
            eprintln!("  {}", path);
        }
        panic!("{} forbidden names in release paths", violations.len());
    }
}

fn check_for_forbidden_names(dir: &Path, violations: &mut Vec<String>) {
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
            let dir_name = path.file_name().unwrap().to_str().unwrap();

            if dir_name == "debug_preview"
                || dir_name == "legacy"
                || dir_name.contains("simulation-debug")
            {
                violations.push(
                    path.strip_prefix(workspace_root)
                        .unwrap()
                        .display()
                        .to_string(),
                );
            }

            check_for_forbidden_names(&path, violations);
        } else if path.extension().is_some_and(|e| e == "rs") {
            let relative_path = path
                .strip_prefix(workspace_root)
                .unwrap()
                .display()
                .to_string();

            if relative_path.contains("demo")
                || relative_path.contains("debug")
                || relative_path.contains("proof")
                || relative_path.contains("vertical_slice_test")
                || relative_path.contains("debug_preview")
                || relative_path.contains("legacy")
                || relative_path.contains("simulation-debug")
            {
                violations.push(relative_path);
            }
        }
    }
}
