//! Проверка: debug модули запрещены в editor release путях

use std::fs;
use std::path::Path;

#[test]
fn no_debug_modules_in_editor_release_path() {
    let workspace_root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .parent()
        .unwrap();

    let mut violations = Vec::new();

    check_for_debug_modules(&workspace_root.join("5.editor"), &mut violations);
    check_for_debug_modules(&workspace_root.join("6.apps/editor"), &mut violations);

    if !violations.is_empty() {
        eprintln!("Debug modules found in editor release paths:");
        for path in &violations {
            eprintln!("  {}", path);
        }
        panic!("{} debug modules in editor release paths", violations.len());
    }
}

fn check_for_debug_modules(dir: &Path, violations: &mut Vec<String>) {
    if !dir.exists() {
        return;
    }

    for entry in fs::read_dir(dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();

        if path.is_dir() {
            check_for_debug_modules(&path, violations);
        } else if path.extension().is_some_and(|e| e == "rs") {
            let file_name = path.file_name().unwrap().to_str().unwrap();
            if file_name.starts_with("debug_") || file_name.contains("_debug.rs") {
                violations.push(path.display().to_string());
            }
        }
    }
}
