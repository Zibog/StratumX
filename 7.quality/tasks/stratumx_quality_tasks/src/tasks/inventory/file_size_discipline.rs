use std::fs;
use std::path::{Path, PathBuf};

/// Maximum lines of code allowed without justification
const MAX_LOC_WITHOUT_JUSTIFICATION: usize = 200;

/// Patterns to identify justification comments
const JUSTIFICATION_PATTERNS: &[&str] = &[
    "JUSTIFICATION:",
    "LOC_JUSTIFICATION:",
    "FILE_SIZE_JUSTIFICATION:",
];

#[derive(Debug, Clone)]
pub struct FileSizeViolation {
    pub path: PathBuf,
    pub loc: usize,
    pub has_justification: bool,
}

/// Check file size discipline across the codebase
pub fn check_file_size_discipline(repo_root: &Path) -> Result<Vec<FileSizeViolation>, String> {
    let mut violations = Vec::new();

    // Check all Rust source files in key directories
    let directories = vec![
        repo_root.join("2.engine"),
        repo_root.join("3.sdk"),
        repo_root.join("4.tooling"),
        repo_root.join("5.editor"),
        repo_root.join("6.apps"),
    ];

    for dir in directories {
        if dir.exists() {
            scan_directory(&dir, &mut violations)?;
        }
    }

    Ok(violations)
}

/// Recursively scan a directory for Rust files
fn scan_directory(dir: &Path, violations: &mut Vec<FileSizeViolation>) -> Result<(), String> {
    let entries = fs::read_dir(dir)
        .map_err(|e| format!("Failed to read directory {:?}: {}", dir, e))?;

    for entry in entries {
        let entry = entry.map_err(|e| format!("Failed to read entry: {}", e))?;
        let path = entry.path();

        if path.is_dir() {
            // Skip target and hidden directories
            if let Some(name) = path.file_name() {
                let name_str = name.to_string_lossy();
                if name_str == "target" || name_str.starts_with('.') {
                    continue;
                }
            }
            scan_directory(&path, violations)?;
        } else if path.extension().and_then(|s| s.to_str()) == Some("rs") {
            check_file(&path, violations)?;
        }
    }

    Ok(())
}

/// Check a single file for size violations
fn check_file(path: &Path, violations: &mut Vec<FileSizeViolation>) -> Result<(), String> {
    let content = fs::read_to_string(path)
        .map_err(|e| format!("Failed to read file {:?}: {}", path, e))?;

    let loc = count_lines_of_code(&content);

    if loc > MAX_LOC_WITHOUT_JUSTIFICATION {
        let has_justification = has_justification_comment(&content);
        violations.push(FileSizeViolation {
            path: path.to_path_buf(),
            loc,
            has_justification,
        });
    }

    Ok(())
}

/// Count lines of code (excluding blank lines and comments)
fn count_lines_of_code(content: &str) -> usize {
    let mut loc = 0;
    let mut in_block_comment = false;

    for line in content.lines() {
        let trimmed = line.trim();

        // Handle block comments
        if trimmed.starts_with("/*") {
            in_block_comment = true;
        }
        if in_block_comment {
            if trimmed.ends_with("*/") || trimmed.contains("*/") {
                in_block_comment = false;
            }
            continue;
        }

        // Skip blank lines and line comments
        if trimmed.is_empty() || trimmed.starts_with("//") {
            continue;
        }

        loc += 1;
    }

    loc
}

/// Check if file has a justification comment
fn has_justification_comment(content: &str) -> bool {
    for pattern in JUSTIFICATION_PATTERNS {
        if content.contains(pattern) {
            return true;
        }
    }
    false
}

/// Format violations for display
pub fn format_violations(violations: &[FileSizeViolation]) -> String {
    if violations.is_empty() {
        return "✓ File size discipline check passed: No violations found\n".to_string();
    }

    let mut output = String::new();
    output.push_str(&format!(
        "\n✗ File size discipline check failed: {} files exceed {} LOC without justification\n\n",
        violations.len(),
        MAX_LOC_WITHOUT_JUSTIFICATION
    ));

    // Separate violations by justification status
    let mut unjustified: Vec<_> = violations.iter().filter(|v| !v.has_justification).collect();
    let mut justified: Vec<_> = violations.iter().filter(|v| v.has_justification).collect();

    // Sort by LOC descending
    unjustified.sort_by(|a, b| b.loc.cmp(&a.loc));
    justified.sort_by(|a, b| b.loc.cmp(&a.loc));

    if !unjustified.is_empty() {
        output.push_str(&format!("Files requiring split or justification ({}):\n", unjustified.len()));
        for violation in unjustified {
            output.push_str(&format!(
                "  - {:?} ({} LOC)\n",
                violation.path, violation.loc
            ));
        }
        output.push('\n');
    }

    if !justified.is_empty() {
        output.push_str(&format!("Files with justification ({}):\n", justified.len()));
        for violation in justified {
            output.push_str(&format!(
                "  - {:?} ({} LOC) [JUSTIFIED]\n",
                violation.path, violation.loc
            ));
        }
        output.push('\n');
    }

    output.push_str("Remediation:\n");
    output.push_str("  1. Split large files by role: ids.rs, types.rs, errors.rs, validation.rs, commands.rs, queries.rs, service.rs\n");
    output.push_str("  2. Add justification comment with pattern: // JUSTIFICATION: <reason>\n");
    output.push_str("  3. Document split in 1.docs/history/SANITATION_LEDGER.md\n");

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_lines_of_code() {
        let content = r#"
// This is a comment
fn main() {
    println!("Hello");
}

/* Block comment
   spanning multiple lines
*/
fn another() {}
"#;
        let loc = count_lines_of_code(content);
        assert_eq!(loc, 4); // fn main, println, }, fn another
    }

    #[test]
    fn test_has_justification_comment() {
        let with_justification = "// JUSTIFICATION: This file is large because...";
        assert!(has_justification_comment(with_justification));

        let without_justification = "// Just a regular comment";
        assert!(!has_justification_comment(without_justification));
    }
}
