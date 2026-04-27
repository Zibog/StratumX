use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct TestFileInventory {
    pub path: PathBuf,
    pub suite: String,
    pub line_count: usize,
    pub test_count: usize,
    pub families: Vec<TestFamily>,
    pub embedded_support: Vec<SupportCodeLocation>,
    pub size_category: SizeCategory,
    pub domain: Domain,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SizeCategory {
    Normal,
    Suspicious,
    RequiresSplit,
    Forbidden,
}

#[derive(Debug, Clone)]
pub struct TestFamily {
    pub name: String,
    pub test_count: usize,
    pub line_range: (usize, usize),
}

#[derive(Debug, Clone)]
pub struct SupportCodeLocation {
    pub kind: SupportCodeKind,
    pub name: String,
    pub line_range: (usize, usize),
}

#[derive(Debug, Clone, PartialEq)]
pub enum SupportCodeKind {
    Builder,
    Fixture,
    Assertion,
    CaseGenerator,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Domain {
    Engine,
    Sdk,
    Editor,
    Tooling,
    Authoring,
    Unknown,
}

impl SizeCategory {
    pub fn from_line_count(line_count: usize) -> Self {
        match line_count {
            0..=300 => SizeCategory::Normal,
            301..=500 => SizeCategory::Suspicious,
            501..=800 => SizeCategory::RequiresSplit,
            _ => SizeCategory::Forbidden,
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            SizeCategory::Normal => "Normal (0-300)",
            SizeCategory::Suspicious => "Suspicious (301-500)",
            SizeCategory::RequiresSplit => "Requires Split (501-800)",
            SizeCategory::Forbidden => "Forbidden (800+)",
        }
    }
}

impl Domain {
    pub fn from_suite_name(suite_name: &str) -> Self {
        if suite_name.contains("engine") {
            Domain::Engine
        } else if suite_name.contains("sdk") {
            Domain::Sdk
        } else if suite_name.contains("editor") {
            Domain::Editor
        } else if suite_name.contains("tool") {
            Domain::Tooling
        } else if suite_name.contains("authoring") || suite_name.contains("world") {
            Domain::Authoring
        } else {
            Domain::Unknown
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            Domain::Engine => "engine",
            Domain::Sdk => "sdk",
            Domain::Editor => "editor",
            Domain::Tooling => "tooling",
            Domain::Authoring => "authoring",
            Domain::Unknown => "unknown",
        }
    }
}
