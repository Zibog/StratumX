//! Проверка: placeholder stubs запрещены в активных путях

use std::fs;
use std::path::Path;

#[test]
fn no_placeholder_stubs_in_active_path() {
    let workspace_root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .parent()
        .unwrap();

    let mut violations = Vec::new();

    check_for_placeholders(&workspace_root.join("2.engine"), &mut violations);
    check_for_placeholders(&workspace_root.join("3.sdk"), &mut violations);
    check_for_placeholders(&workspace_root.join("4.tooling"), &mut violations);
    check_for_placeholders(&workspace_root.join("5.editor"), &mut violations);
    check_for_placeholders(&workspace_root.join("6.apps"), &mut violations);

    if !violations.is_empty() {
        eprintln!("Placeholder stubs found in active paths:");
        for (path, line, content) in &violations {
            eprintln!("  {}:{}", path, line);
            eprintln!("    {}", content.trim());
        }
        panic!("{} placeholder stubs in active paths", violations.len());
    }
}

fn check_for_placeholders(dir: &Path, violations: &mut Vec<(String, usize, String)>) {
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
            check_for_placeholders(&path, violations);
        } else if path.extension().map_or(false, |e| e == "rs") {
            if let Ok(content) = fs::read_to_string(&path) {
                let relative_path = path
                    .strip_prefix(workspace_root)
                    .unwrap()
                    .display()
                    .to_string();

                for (line_num, line) in content.lines().enumerate() {
                    let trimmed = line.trim();

                    // Skip legitimate function/field names containing "placeholder"
                    if trimmed.starts_with("pub fn ")
                        || trimmed.starts_with("fn ")
                        || trimmed.starts_with("pub ")
                        || trimmed.contains(": bool,")
                        || trimmed.contains("Placeholder,")
                        || trimmed.contains("placeholder_or_live_surface")
                        || trimmed.contains("create_placeholder_terrain_mesh")
                    {
                        continue;
                    }

                    let line_lower = line.to_lowercase();

                    // Only catch comment-style placeholders
                    if (line_lower.contains("placeholder")
                        && (trimmed.starts_with("//") || trimmed.starts_with("/*")))
                        || (line_lower.contains("simplified implementation")
                            && trimmed.starts_with("//"))
                        || (line_lower.contains("for now") && trimmed.starts_with("//"))
                        || (line_lower.contains("not yet wired") && trimmed.starts_with("//"))
                        || line_lower.contains("todo:")
                        || line_lower.contains("fixme:")
                        || line_lower.contains("unimplemented!")
                        || line_lower.contains("panic!")
                    {
                        violations.push((relative_path.clone(), line_num + 1, line.to_string()));
                    }
                }
            }
        }
    }
}
