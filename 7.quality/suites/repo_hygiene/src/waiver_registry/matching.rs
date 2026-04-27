use super::{WaiverEntry, WaiverRegistry};
use crate::models::HygieneRule;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

pub fn is_waived(registry: &WaiverRegistry, rule: &HygieneRule, file: &Path) -> bool {
    let file_str = normalize_path(file);
    entries_for_rule(registry, rule)
        .iter()
        .any(|entry| normalize_waiver_path(&entry.path) == file_str)
}

pub fn get_waivers_for_rule(
    registry: &WaiverRegistry,
    rule: &HygieneRule,
) -> HashMap<PathBuf, String> {
    entries_for_rule(registry, rule)
        .iter()
        .map(|entry| (PathBuf::from(&entry.path), entry.justification.clone()))
        .collect()
}

fn entries_for_rule<'a>(registry: &'a WaiverRegistry, rule: &HygieneRule) -> &'a [WaiverEntry] {
    match rule {
        HygieneRule::LineLimit { .. } => &registry.line_limit_waivers,
        HygieneRule::AllowAttribute { .. } => &registry.allow_attr_waivers,
        HygieneRule::TestFileInSrc => &registry.test_in_src_waivers,
        HygieneRule::ExecuteBridge => &registry.execute_bridge_waivers,
        _ => &[],
    }
}

fn normalize_path(path: &Path) -> String {
    path.to_string_lossy().replace('\\', "/")
}

fn normalize_waiver_path(path: &str) -> String {
    path.replace('\\', "/")
}
