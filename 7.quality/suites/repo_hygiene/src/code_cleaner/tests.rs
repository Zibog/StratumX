use super::CodeCleaner;
use std::path::PathBuf;

#[test]
fn test_code_cleaner_creation() {
    let cleaner = CodeCleaner::new(PathBuf::from("/test/repo"));
    assert_eq!(cleaner.repo_root, PathBuf::from("/test/repo"));
}

#[test]
fn test_extract_host_pattern() {
    let cleaner = CodeCleaner::new(PathBuf::from("/test"));

    let pattern = cleaner.extract_host_pattern("    self.host.save_file(path, content);");
    assert_eq!(
        pattern,
        Some("self.host.save_file(path, content)".to_string())
    );

    let pattern = cleaner.extract_host_pattern("let result = self.host.open_file_dialog();");
    assert_eq!(pattern, Some("self.host.open_file_dialog()".to_string()));
}

#[test]
fn test_suggest_host_bypass_remediation() {
    let cleaner = CodeCleaner::new(PathBuf::from("/test"));

    let suggestion = cleaner.suggest_host_bypass_remediation("self.host.save_file(path, content)");
    assert!(suggestion.contains("SaveFile"));

    let suggestion = cleaner.suggest_host_bypass_remediation("self.host.open_file_dialog()");
    assert!(suggestion.contains("dialog"));
}

#[test]
fn test_detect_mixed_concerns() {
    let cleaner = CodeCleaner::new(PathBuf::from("/test"));

    let content = r#"
        fn register_all() {
            register_command("save", SaveCommand);
            register_panel("project", ProjectPanel);
            register_tool("debugger", DebuggerTool);
        }
    "#;

    let concerns = cleaner.detect_mixed_concerns(content);
    assert!(concerns.len() >= 2);
    assert!(concerns.iter().any(|c| c.contains("Command")));
    assert!(concerns.iter().any(|c| c.contains("Panel")));
}
