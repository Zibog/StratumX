//! Проверка: нет missing module references
//! Все mod declarations должны указывать на существующие файлы

use std::fs;
use std::path::Path;

#[test]
fn no_missing_module_references() {
    let workspace_root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .parent()
        .unwrap();

    let mut violations = Vec::new();

    check_module_references(&workspace_root.join("2.engine"), &mut violations);
    check_module_references(&workspace_root.join("3.sdk"), &mut violations);
    check_module_references(&workspace_root.join("4.tooling"), &mut violations);
    check_module_references(&workspace_root.join("5.editor"), &mut violations);
    check_module_references(&workspace_root.join("6.apps"), &mut violations);

    if !violations.is_empty() {
        eprintln!("Missing module references found:");
        for (file_path, line, module_name, expected_paths) in &violations {
            eprintln!("  {}:{}", file_path, line);
            eprintln!("    mod {};", module_name);
            eprintln!("    Expected one of:");
            for expected in expected_paths {
                eprintln!("      {}", expected);
            }
        }
        panic!("{} missing module references", violations.len());
    }
}

fn check_module_references(dir: &Path, violations: &mut Vec<(String, usize, String, Vec<String>)>) {
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
            check_module_references(&path, violations);
        } else if path.extension().map_or(false, |e| e == "rs") {
            if let Ok(content) = fs::read_to_string(&path) {
                let relative_path = path
                    .strip_prefix(workspace_root)
                    .unwrap()
                    .display()
                    .to_string();

                let parent_dir = path.parent().unwrap();

                for (line_num, line) in content.lines().enumerate() {
                    let trimmed = line.trim();

                    // Match "mod module_name;" or "pub mod module_name;"
                    if let Some(module_name) = extract_module_declaration(trimmed) {
                        // Check if module file exists in same directory or as subdirectory
                        let module_file_sibling = parent_dir.join(format!("{}.rs", module_name));
                        let module_dir_child = parent_dir.join(&module_name).join("mod.rs");

                        // For non-mod.rs files, also check in subdirectory named after the file
                        let file_stem = path.file_stem().unwrap().to_str().unwrap();
                        let module_file_in_subdir = if file_stem != "mod" {
                            Some(
                                parent_dir
                                    .join(file_stem)
                                    .join(format!("{}.rs", module_name)),
                            )
                        } else {
                            None
                        };
                        let module_dir_in_subdir = if file_stem != "mod" {
                            Some(parent_dir.join(file_stem).join(&module_name).join("mod.rs"))
                        } else {
                            None
                        };

                        let exists = module_file_sibling.exists()
                            || module_dir_child.exists()
                            || module_file_in_subdir.as_ref().map_or(false, |p| p.exists())
                            || module_dir_in_subdir.as_ref().map_or(false, |p| p.exists());

                        if !exists {
                            let mut expected_paths = Vec::new();
                            expected_paths.push(
                                module_file_sibling
                                    .strip_prefix(workspace_root)
                                    .unwrap()
                                    .display()
                                    .to_string(),
                            );
                            expected_paths.push(
                                module_dir_child
                                    .strip_prefix(workspace_root)
                                    .unwrap()
                                    .display()
                                    .to_string(),
                            );
                            if let Some(p) = module_file_in_subdir {
                                expected_paths.push(
                                    p.strip_prefix(workspace_root)
                                        .unwrap()
                                        .display()
                                        .to_string(),
                                );
                            }
                            if let Some(p) = module_dir_in_subdir {
                                expected_paths.push(
                                    p.strip_prefix(workspace_root)
                                        .unwrap()
                                        .display()
                                        .to_string(),
                                );
                            }

                            violations.push((
                                relative_path.clone(),
                                line_num + 1,
                                module_name.to_string(),
                                expected_paths,
                            ));
                        }
                    }
                }
            }
        }
    }
}

fn extract_module_declaration(line: &str) -> Option<&str> {
    let line = line.trim();

    // Skip comments
    if line.starts_with("//") || line.starts_with("/*") {
        return None;
    }

    // Match "mod module_name;" or "pub mod module_name;"
    if line.starts_with("mod ") || line.starts_with("pub mod ") {
        let after_mod = if line.starts_with("pub mod ") {
            line.strip_prefix("pub mod ")?
        } else {
            line.strip_prefix("mod ")?
        };

        // Extract module name before semicolon or opening brace
        let module_name = after_mod.split(';').next()?.split('{').next()?.trim();

        // Skip inline module definitions (mod name { ... })
        if after_mod.contains('{') {
            return None;
        }

        if !module_name.is_empty() {
            return Some(module_name);
        }
    }

    None
}
