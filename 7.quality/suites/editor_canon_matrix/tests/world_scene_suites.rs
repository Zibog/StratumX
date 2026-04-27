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

fn world_scene_suites_case_strategy() -> impl Strategy<Value = usize> {
    0usize..100
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(100))]
    #[test]
    fn world_scene_suites_all_cases(case in world_scene_suites_case_strategy()) {
        use common::*;
        let mut editor = editor();
        editor.create_world_anchor(format!("w-{case}")).unwrap();
        editor.create_scene_entity(format!("s-{case}")).unwrap();
        assert!(!editor.world_suite.world_roots.is_empty());
        assert!(!editor.scene_suite.scene_entities.is_empty());
    }
}
