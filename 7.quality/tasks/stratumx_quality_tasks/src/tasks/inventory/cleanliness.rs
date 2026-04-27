use std::path::{Path, PathBuf};
use std::fs;

/// Patterns for temporary/stray files that should not be in root
const STRAY_FILE_PATTERNS: &[&str] = &[
    r"\.orig$",
    r"\.bak$",
    r"_PLAN\.md$",
    r"_ROADMAP\.md$",
    r"_notes\.txt$",
];

/// Directories that are tool-specific and should be documented or removed
const TOOL_CONFIG_DIRS: &[&str] = &[
    ".qwen",
    ".cursor",
    ".windsurf",
];

#[derive(Debug)]
pub struct CleanlinessViolation {
    pub path: PathBuf,
    pub violation_type: ViolationType,
}

#[derive(Debug)]
pub enum ViolationType {
    StrayFile,
    UndocumentedToolDir,
}

pub fn check_root_cleanliness(repo_root: &Path) -> Result<Vec<CleanlinessViolation>, String> {
    let mut violations = Vec::new();

    // Check for stray files in root
    let entries = fs::read_dir(repo_root)
        .map_err(|e| format!("Failed to read root directory: {}", e))?;

    for entry in entries {
        let entry = entry.map_err(|e| format!("Failed to read directory entry: {}", e))?;
        let path = entry.path();
        let file_name = path.file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("");

        // Check if it's a file matching stray patterns
        if path.is_file() {
            for pattern in STRAY_FILE_PATTERNS {
                let re = regex::Regex::new(pattern)
                    .map_err(|e| format!("Invalid regex pattern: {}", e))?;
                if re.is_match(file_name) {
                    violations.push(CleanlinessViolation {
                        path: path.clone(),
                        violation_type: ViolationType::StrayFile,
                    });
                    break;
                }
            }
        }

        // Check for undocumented tool config directories
        if path.is_dir() && TOOL_CONFIG_DIRS.contains(&file_name) {
            violations.push(CleanlinessViolation {
                path: path.clone(),
                violation_type: ViolationType::UndocumentedToolDir,
            });
        }
    }

    Ok(violations)
}

pub fn format_violations(violations: &[CleanlinessViolation]) -> String {
    if violations.is_empty() {
        return "✓ Root directory is clean (no stray files or undocumented tool directories)".to_string();
    }

    let mut output = String::new();
    output.push_str(&format!("✗ Found {} root cleanliness violations:\n", violations.len()));
    
    for violation in violations {
        let path_str = violation.path.display();
        match violation.violation_type {
            ViolationType::StrayFile => {
                output.push_str(&format!("  - Stray file: {}\n", path_str));
                output.push_str("    → Should be moved to 1.docs/history/ or 1.docs/plans/archive/\n");
            }
            ViolationType::UndocumentedToolDir => {
                output.push_str(&format!("  - Undocumented tool directory: {}\n", path_str));
                output.push_str("    → Should be documented in README or removed if not officially supported\n");
            }
        }
    }

    output
}
