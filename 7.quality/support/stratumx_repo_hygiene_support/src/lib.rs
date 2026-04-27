pub mod test_repo_fixture;

use std::fs;
use std::path::{Path, PathBuf};

pub use test_repo_fixture::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InlineTestDebtKind {
    MisplacedTestFile,
    PropertyMatrixInSrc,
    InvariantSuiteInSrc,
    SmokeOrIntegrationInSrc,
    HeavyInlineTestModule,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InlineTestDebt {
    pub file: PathBuf,
    pub line: usize,
    pub kind: InlineTestDebtKind,
    pub snippet: String,
    pub test_count: usize,
    pub line_span: usize,
}

pub fn workspace_root_from_manifest_dir(manifest_dir: &str) -> PathBuf {
    PathBuf::from(manifest_dir)
        .join("../../..")
        .canonicalize()
        .expect("workspace root")
}

pub fn production_rust_files(root: &Path) -> Vec<PathBuf> {
    let mut files = Vec::new();
    for relative in ["2.engine", "3.sdk", "4.tooling", "5.editor", "6.apps"] {
        let dir = root.join(relative);
        if dir.exists() {
            walk_src_rust_files(&dir, &mut files);
        }
    }
    files
}

pub fn scan_inline_test_debt(root: &Path) -> Vec<InlineTestDebt> {
    production_rust_files(root)
        .into_iter()
        .flat_map(|path| {
            let content = fs::read_to_string(&path).unwrap_or_default();
            find_inline_test_debt_in_file(&path, &content)
        })
        .collect()
}

pub fn find_inline_test_debt_in_file(path: &Path, content: &str) -> Vec<InlineTestDebt> {
    if !is_production_src_rust_file(path) {
        return Vec::new();
    }

    let mut debts = Vec::new();
    let lines: Vec<&str> = content.lines().collect();
    let file_name = path
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or_default();
    let lower_file_name = file_name.to_ascii_lowercase();
    let test_count = content.matches("#[test]").count();
    let has_test_module = lines
        .iter()
        .any(|line| line.trim().starts_with("mod tests"));
    let first_test_line = lines
        .iter()
        .position(|line| {
            let trimmed = line.trim();
            trimmed == "#[cfg(test)]" || trimmed == "#[test]" || trimmed.starts_with("mod tests")
        })
        .map(|index| index + 1)
        .unwrap_or(1);
    let line_span = lines.len().saturating_sub(first_test_line) + 1;
    let first_snippet = lines
        .get(first_test_line.saturating_sub(1))
        .copied()
        .unwrap_or_default()
        .trim()
        .to_string();

    if file_name.ends_with(".test.rs")
        || file_name.ends_with("_tests.rs")
        || file_name == "tests.rs"
    {
        debts.push(InlineTestDebt {
            file: path.to_path_buf(),
            line: 1,
            kind: InlineTestDebtKind::MisplacedTestFile,
            snippet: file_name.to_string(),
            test_count,
            line_span,
        });
    }

    if content.contains("proptest!") || content.contains("prop_compose!") {
        debts.push(InlineTestDebt {
            file: path.to_path_buf(),
            line: first_test_line,
            kind: InlineTestDebtKind::PropertyMatrixInSrc,
            snippet: first_snippet.clone(),
            test_count,
            line_span,
        });
    }

    if content.contains("invariant!") {
        debts.push(InlineTestDebt {
            file: path.to_path_buf(),
            line: first_test_line,
            kind: InlineTestDebtKind::InvariantSuiteInSrc,
            snippet: first_snippet.clone(),
            test_count,
            line_span,
        });
    }

    let has_named_smoke_or_integration_test = lines.iter().any(|line| {
        let trimmed = line.trim().to_ascii_lowercase();
        trimmed.starts_with("mod smoke")
            || trimmed.starts_with("mod integration")
            || trimmed.starts_with("fn smoke")
            || trimmed.starts_with("fn integration")
    });
    if (lower_file_name.contains("smoke")
        || lower_file_name.contains("integration")
        || has_named_smoke_or_integration_test)
        && (content.contains("#[cfg(test)]") || content.contains("#[test]"))
    {
        debts.push(InlineTestDebt {
            file: path.to_path_buf(),
            line: first_test_line,
            kind: InlineTestDebtKind::SmokeOrIntegrationInSrc,
            snippet: first_snippet.clone(),
            test_count,
            line_span,
        });
    }

    if has_test_module && test_count > 0 && (test_count > 2 || line_span > 30) {
        debts.push(InlineTestDebt {
            file: path.to_path_buf(),
            line: first_test_line,
            kind: InlineTestDebtKind::HeavyInlineTestModule,
            snippet: first_snippet,
            test_count,
            line_span,
        });
    }

    debts
}

pub fn declared_test_count(root: &Path) -> usize {
    let suites = root.join("7.quality").join("suites");
    let mut files = Vec::new();
    walk_rs_files(&suites, &mut files);
    files
        .into_iter()
        .filter_map(|file| fs::read_to_string(file).ok())
        .map(|content| {
            content
                .lines()
                .filter(|line| {
                    let trimmed = line.trim();
                    trimmed == "#[test]" || trimmed.starts_with("proptest!")
                })
                .count()
        })
        .sum()
}

pub fn quality_suite_names(root: &Path) -> Vec<String> {
    let suites = root.join("7.quality").join("suites");
    let mut names = Vec::new();
    if let Ok(entries) = fs::read_dir(&suites) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                if let Some(name) = path.file_name().and_then(|name| name.to_str()) {
                    names.push(name.to_string());
                }
            }
        }
    }
    names.sort();
    names
}

fn walk_src_rust_files(dir: &Path, files: &mut Vec<PathBuf>) {
    let entries = match fs::read_dir(dir) {
        Ok(entries) => entries,
        Err(_) => return,
    };

    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            if let Some(name) = path.file_name().and_then(|name| name.to_str()) {
                if name.starts_with('.') || name == "target" || name == "7.quality" {
                    continue;
                }
            }
            walk_src_rust_files(&path, files);
        } else if is_production_src_rust_file(&path) {
            files.push(path);
        }
    }
}

fn walk_rs_files(dir: &Path, files: &mut Vec<PathBuf>) {
    let entries = match fs::read_dir(dir) {
        Ok(entries) => entries,
        Err(_) => return,
    };

    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            walk_rs_files(&path, files);
        } else if path.extension().and_then(|ext| ext.to_str()) == Some("rs") {
            files.push(path);
        }
    }
}

fn is_production_src_rust_file(path: &Path) -> bool {
    path.extension().and_then(|ext| ext.to_str()) == Some("rs")
        && path
            .ancestors()
            .any(|ancestor| ancestor.file_name().and_then(|name| name.to_str()) == Some("src"))
        && !path.ancestors().any(|ancestor| {
            ancestor.file_name().and_then(|name| name.to_str()) == Some("7.quality")
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tiny_inline_tests_are_allowed_but_heavy_mod_tests_are_flagged() {
        let tiny = r#"
            fn parse_mode() -> u8 { 1 }
            #[cfg(test)]
            mod tests {
                #[test]
                fn round_trip() {}
            }
        "#;
        let heavy = r#"
            #[cfg(test)]
            mod tests {
                #[test]
                fn a() {}
                #[test]
                fn b() {}
                #[test]
                fn c() {}
                fn helper_1() {}
                fn helper_2() {}
                fn helper_3() {}
                fn helper_4() {}
                fn helper_5() {}
            }
        "#;

        assert!(find_inline_test_debt_in_file(Path::new("5.editor/foo/src/a.rs"), tiny).is_empty());
        assert!(
            !find_inline_test_debt_in_file(Path::new("5.editor/foo/src/a.rs"), heavy).is_empty()
        );
    }

    #[test]
    fn helper_only_cfg_test_sections_are_not_flagged_as_heavy() {
        let helper_only = r#"
            pub struct ActionContext;

            impl ActionContext {
                #[cfg(test)]
                pub fn test_empty() -> Self { Self }

                #[cfg(test)]
                pub fn test_with_world() -> Self { Self }
            }
        "#;

        let debts = find_inline_test_debt_in_file(Path::new("5.editor/foo/src/a.rs"), helper_only);
        assert!(debts.is_empty());
    }

    #[test]
    fn comments_do_not_trigger_smoke_or_integration_detection() {
        let content = r#"
            // Integration tests are in the tests/ directory.
            pub struct Registry;

            #[cfg(test)]
            mod tests {
                #[test]
                fn round_trip() {}
            }
        "#;

        let debts = find_inline_test_debt_in_file(Path::new("5.editor/foo/src/lib.rs"), content);
        assert!(!debts
            .iter()
            .any(|debt| debt.kind == InlineTestDebtKind::SmokeOrIntegrationInSrc));
    }
}
