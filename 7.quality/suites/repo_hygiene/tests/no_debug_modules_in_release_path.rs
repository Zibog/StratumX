//! Проверка: debug/demo модули не должны быть в release путях

use std::fs;
use std::path::Path;

#[test]
fn no_debug_modules_in_release_path() {
    let workspace_root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .parent()
        .unwrap();

    let mut violations = Vec::new();

    check_for_debug_modules(&workspace_root.join("2.engine"), &mut violations);
    check_for_debug_modules(&workspace_root.join("3.sdk"), &mut violations);
    check_for_debug_modules(&workspace_root.join("4.tooling"), &mut violations);
    check_for_debug_modules(&workspace_root.join("5.editor"), &mut violations);

    if !violations.is_empty() {
        eprintln!("Debug modules found in release paths:");
        for path in &violations {
            eprintln!("  {}", path);
        }
        panic!("{} debug modules in release paths", violations.len());
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
            if file_name.contains("debug") || file_name.contains("_dbg") {
                violations.push(
                    path.strip_prefix(dir.parent().unwrap().parent().unwrap())
                        .unwrap()
                        .display()
                        .to_string(),
                );
            }
        }
    }
}
