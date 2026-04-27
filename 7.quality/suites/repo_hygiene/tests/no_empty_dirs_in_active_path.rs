//! Проверка: пустые директории не должны быть в активных путях

use std::fs;
use std::path::Path;

#[test]
fn no_empty_dirs_in_active_path() {
    let workspace_root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .parent()
        .unwrap();

    let mut violations = Vec::new();

    // Проверяем активные пути
    check_for_empty_dirs(&workspace_root.join("2.engine"), &mut violations);
    check_for_empty_dirs(&workspace_root.join("3.sdk"), &mut violations);
    check_for_empty_dirs(&workspace_root.join("4.tooling"), &mut violations);
    check_for_empty_dirs(&workspace_root.join("5.editor"), &mut violations);
    check_for_empty_dirs(&workspace_root.join("6.apps"), &mut violations);

    if !violations.is_empty() {
        eprintln!("Empty directories found in active paths:");
        for path in &violations {
            eprintln!("  {}", path);
        }
        panic!("{} empty directories in active paths", violations.len());
    }
}

fn check_for_empty_dirs(dir: &Path, violations: &mut Vec<String>) {
    if !dir.exists() {
        return;
    }

    for entry in fs::read_dir(dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();

        if path.is_dir() {
            // Рекурсивно проверяем поддиректории
            check_for_empty_dirs(&path, violations);

            // Проверяем, является ли директория пустой
            if is_empty_dir(&path) {
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

fn is_empty_dir(dir: &Path) -> bool {
    if !dir.is_dir() {
        return false;
    }

    match fs::read_dir(dir) {
        Ok(mut entries) => entries.next().is_none(),
        Err(_) => false, // Если не удалось прочитать директорию, считаем ее не пустой
    }
}
