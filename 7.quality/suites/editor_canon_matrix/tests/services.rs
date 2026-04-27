#![allow(
    unused_imports,
    unused_mut,
    unused_variables,
    clippy::manual_is_multiple_of,
    clippy::if_same_then_else,
    clippy::assertions_on_constants,
    dead_code
)]
mod common;
use common::*;
use proptest::prelude::*;
use stratumx_test_support::*;

fn services_case_strategy() -> impl Strategy<Value = usize> {
    0usize..100
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(100))]
    #[test]
    fn services_all_cases(case in services_case_strategy()) {
        use common::*;
        let editor = editor();
        assert_eq!(
            editor.project_bootstrap_service.active_project,
            "Editor Test"
        );
        assert!(editor
            .plugin_host
            .registered_plugins
            .iter()
            .any(|p| p.contains("core")));
        assert!(editor.template_service.template_count >= 1);
    }
}
