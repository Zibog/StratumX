//! Проверка: BOM (Byte Order Mark) запрещен в .rs файлах
//! BOM может вызывать проблемы с IDE, диффами и компиляцией

use std::fs;
use std::path::Path;

#[test]
fn no_bom_in_rust_files() {
    let workspace_root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .parent()
        .unwrap();

    let mut violations = Vec::new();

    check_for_bom(&workspace_root.join("2.engine"), &mut violations);
    check_for_bom(&workspace_root.join("3.sdk"), &mut violations);
    check_for_bom(&workspace_root.join("4.tooling"), &mut violations);
    check_for_bom(&workspace_root.join("5.editor"), &mut violations);
    check_for_bom(&workspace_root.join("6.apps"), &mut violations);
    check_for_bom(&workspace_root.join("7.quality"), &mut violations);

    if !violations.is_empty() {
        eprintln!("BOM found in Rust files:");
        for path in &violations {
            eprintln!("  {}", path);
        }
        eprintln!("\nTo fix, remove BOM from these files using:");
        eprintln!("  PowerShell: $content = Get-Content -Raw -Encoding UTF8 <file>; [System.IO.File]::WriteAllText(<file>, $content, (New-Object System.Text.UTF8Encoding $false))");
        panic!("{} files with BOM", violations.len());
    }
}

fn check_for_bom(dir: &Path, violations: &mut Vec<String>) {
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
            check_for_bom(&path, violations);
        } else if path.extension().map_or(false, |e| e == "rs") {
            if let Ok(bytes) = fs::read(&path) {
                // Check for UTF-8 BOM: EF BB BF
                if bytes.len() >= 3 && bytes[0] == 0xEF && bytes[1] == 0xBB && bytes[2] == 0xBF {
                    let relative_path = path
                        .strip_prefix(workspace_root)
                        .unwrap()
                        .display()
                        .to_string();
                    violations.push(relative_path);
                }
            }
        }
    }
}
