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

fn domain_suites_case_strategy() -> impl Strategy<Value = usize> {
    0usize..100
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(100))]
    #[test]
    fn domain_suites_all_cases(case in domain_suites_case_strategy()) {
        use common::*;
        let mut editor = editor();
        editor.create_material(format!("m-{case}")).unwrap();
        editor.create_logic_node(format!("q-{case}")).unwrap();
        assert!(!editor.material_suite.materials.is_empty());
        assert!(!editor.quest_suite.quest_nodes.is_empty());
        assert!(!editor.simulation_suite.logic_nodes.is_empty());
    }
}
