use super::models::{SupportCodeKind, SupportCodeLocation, TestFamily};
use std::collections::HashMap;

pub fn count_non_blank_non_comment_lines(content: &str) -> usize {
    content
        .lines()
        .filter(|line| {
            let trimmed = line.trim();
            !trimmed.is_empty() && !trimmed.starts_with("//")
        })
        .count()
}

pub fn identify_test_families(content: &str) -> (usize, Vec<TestFamily>) {
    let mut test_count = 0;
    let mut family_map: HashMap<String, Vec<usize>> = HashMap::new();
    let mut in_test_attr = false;

    for (line_num, line) in content.lines().enumerate() {
        if line.trim() == "#[test]" {
            in_test_attr = true;
            continue;
        }

        if in_test_attr || line.trim().starts_with("fn test_") {
            if let Some(fn_start) = line.find("fn ") {
                let after_fn = &line[fn_start + 3..];
                if let Some(paren_pos) = after_fn.find('(') {
                    let test_name = after_fn[..paren_pos].trim();
                    test_count += 1;

                    let family_name = extract_family_name(test_name);
                    family_map
                        .entry(family_name)
                        .or_default()
                        .push(line_num + 1);
                }
            }
            in_test_attr = false;
        }
    }

    let mut families: Vec<TestFamily> = family_map
        .into_iter()
        .filter(|(_, lines)| lines.len() > 1)
        .map(|(name, lines)| {
            let min_line = *lines.iter().min().unwrap_or(&0);
            let max_line = *lines.iter().max().unwrap_or(&0);
            TestFamily {
                name,
                test_count: lines.len(),
                line_range: (min_line, max_line),
            }
        })
        .collect();

    families.sort_by(|a, b| b.test_count.cmp(&a.test_count));

    (test_count, families)
}

pub fn detect_support_code(content: &str) -> Vec<SupportCodeLocation> {
    let mut support_code = Vec::new();

    for (line_num, line) in content.lines().enumerate() {
        let trimmed = line.trim();

        if trimmed.starts_with("fn ") || trimmed.starts_with("pub fn ") {
            if let Some(fn_start) = trimmed.find("fn ") {
                let after_fn = &trimmed[fn_start + 3..];
                if let Some(paren_pos) = after_fn.find('(') {
                    let fn_name = after_fn[..paren_pos].trim();
                    let kind = if fn_name.contains("build") || fn_name.contains("create") {
                        Some(SupportCodeKind::Builder)
                    } else if fn_name.contains("assert") || fn_name.contains("check") {
                        Some(SupportCodeKind::Assertion)
                    } else if fn_name.contains("generate") || fn_name.contains("cases") {
                        Some(SupportCodeKind::CaseGenerator)
                    } else {
                        None
                    };

                    if let Some(kind) = kind {
                        support_code.push(SupportCodeLocation {
                            kind,
                            name: fn_name.to_string(),
                            line_range: (line_num + 1, line_num + 1),
                        });
                    }
                }
            }
        }

        if (trimmed.starts_with("const ") || trimmed.starts_with("static "))
            && !trimmed.contains("fn ")
        {
            if let Some(name_end) = trimmed.find(':').or_else(|| trimmed.find('=')) {
                let name_part = &trimmed[..name_end];
                if let Some(name_start) = name_part.rfind(' ') {
                    let name = name_part[name_start + 1..].trim();
                    support_code.push(SupportCodeLocation {
                        kind: SupportCodeKind::Fixture,
                        name: name.to_string(),
                        line_range: (line_num + 1, line_num + 1),
                    });
                }
            }
        }
    }

    support_code
}

fn extract_family_name(test_name: &str) -> String {
    let name = test_name.strip_prefix("test_").unwrap_or(test_name);

    if let Some(pos) = name.rfind('_') {
        let after_underscore = &name[pos + 1..];
        if after_underscore.chars().all(|c| c.is_ascii_digit()) {
            return name[..pos].to_string();
        }
    }

    name.to_string()
}
