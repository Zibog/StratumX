use super::WaiverRegistry;
use crate::HygieneRule;
use std::path::Path;

#[test]
fn test_empty_registry() {
    let registry = WaiverRegistry::new();
    assert_eq!(registry.line_limit_waivers.len(), 0);
    assert_eq!(registry.allow_attr_waivers.len(), 0);
    assert_eq!(registry.test_in_src_waivers.len(), 0);
    assert_eq!(registry.execute_bridge_waivers.len(), 0);
}

#[test]
fn test_is_waived_returns_false_for_empty_registry() {
    let registry = WaiverRegistry::new();
    let rule = HygieneRule::LineLimit {
        limit: 200,
        actual: 250,
    };
    let file = Path::new("some/file.rs");

    assert!(!registry.is_waived(&rule, file));
}
