use super::models::{SizeCategory, SupportCodeLocation, TestFileInventory};
use std::collections::HashMap;

pub fn giant_files(inventory: &[TestFileInventory]) -> Vec<&TestFileInventory> {
    inventory
        .iter()
        .filter(|file| matches!(file.size_category, SizeCategory::Forbidden))
        .collect()
}

pub fn files_with_support(inventory: &[TestFileInventory]) -> Vec<&TestFileInventory> {
    inventory
        .iter()
        .filter(|file| !file.embedded_support.is_empty())
        .collect()
}

pub fn count_by_size_category(inventory: &[TestFileInventory]) -> HashMap<String, usize> {
    let mut counts = HashMap::new();
    for file in inventory {
        *counts
            .entry(file.size_category.as_str().to_string())
            .or_insert(0) += 1;
    }
    counts
}

pub fn count_support_by_kind(files_with_support: &[&TestFileInventory]) -> HashMap<String, usize> {
    let mut counts = HashMap::new();
    for file in files_with_support {
        for support in &file.embedded_support {
            *counts.entry(format!("{:?}", support.kind)).or_insert(0) += 1;
        }
    }
    counts
}

pub fn group_by_suite(inventory: &[TestFileInventory]) -> HashMap<String, Vec<&TestFileInventory>> {
    let mut by_suite = HashMap::new();
    for file in inventory {
        by_suite
            .entry(file.suite.clone())
            .or_insert_with(Vec::new)
            .push(file);
    }
    by_suite
}

pub fn count_by_domain(
    inventory: &[TestFileInventory],
) -> HashMap<String, Vec<&TestFileInventory>> {
    let mut by_domain = HashMap::new();
    for file in inventory {
        by_domain
            .entry(file.domain.as_str().to_string())
            .or_insert_with(Vec::new)
            .push(file);
    }
    by_domain
}

pub fn total_tests(inventory: &[TestFileInventory]) -> usize {
    inventory.iter().map(|file| file.test_count).sum()
}

pub fn total_lines(inventory: &[TestFileInventory]) -> usize {
    inventory.iter().map(|file| file.line_count).sum()
}

pub fn average_lines(inventory: &[TestFileInventory]) -> usize {
    if inventory.is_empty() {
        0
    } else {
        total_lines(inventory) / inventory.len()
    }
}

pub fn giant_family_files<'a>(
    giant_files: &'a [&'a TestFileInventory],
) -> Vec<&'a TestFileInventory> {
    giant_files
        .iter()
        .copied()
        .filter(|file| !file.families.is_empty())
        .collect()
}

pub fn support_locations(file: &TestFileInventory) -> &[SupportCodeLocation] {
    &file.embedded_support
}
