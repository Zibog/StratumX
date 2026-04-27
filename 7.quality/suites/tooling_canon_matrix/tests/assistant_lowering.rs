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

fn assistant_lowering_case_strategy() -> impl Strategy<Value = usize> {
    0usize..100
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(100))]
    #[test]
    fn assistant_lowering_all_cases(case in assistant_lowering_case_strategy()) {
        use common::*;
        let runtime = runtime();
        let plan = runtime.plan_goal(format!("goal-{case}"));
        assert_eq!(plan.suggested_commands.len(), 3);
        assert!(matches!(
            &plan.suggested_commands[0],
            ToolCommand::CreateObject { label, class: ObjectClass::World }
                if label == &format!("world-goal-{case}")
        ));
        assert!(matches!(
            &plan.suggested_commands[1],
            ToolCommand::CreateObject { label, class: ObjectClass::Scene }
                if label == &format!("scene-goal-{case}")
        ));
        assert!(matches!(
            &plan.suggested_commands[2],
            ToolCommand::CreateObject { label, class: ObjectClass::Logic }
                if label == &format!("logic-goal-{case}")
        ));
    }
}
