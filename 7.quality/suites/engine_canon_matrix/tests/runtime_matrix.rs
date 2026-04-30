#![allow(unused_imports, unused_mut, unused_variables)]
mod common;
use common::*;
use proptest::prelude::*;

fn runtime_apply_tick_strategy() -> impl Strategy<Value = usize> {
    0usize..40
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(40))]
    #[test]
    fn runtime_all_apply_ticks(case in runtime_apply_tick_strategy()) {
        let _case = case;
        let mut rt = RuntimeKernel::new(
            WorldState::new(),
            RuntimeConfig {
                profile: RuntimeProfile::Interactive60,
                max_apply_segments_per_tick: 4,
                publish_passes: 1,
            },
        );
        rt.enqueue_apply_segment(ApplySegment {
            region_key: (0, 0, 0),
            family_tags: vec![1],
        })
        .unwrap();
        rt.enqueue_presentable_frame(PresentableFrame {
            frame_id: 1,
            visibility_freshness_frames: 1,
        })
        .unwrap();
        let res = rt.run_tick().unwrap();
        prop_assert_eq!(res.applied_segments, 1);
    }
}

#[test]
fn runtime_digest_is_deterministic_for_equivalent_apply_batches() {
    let config = RuntimeConfig {
        profile: RuntimeProfile::Interactive60,
        max_apply_segments_per_tick: 4,
        publish_passes: 1,
    };
    let mut left = RuntimeKernel::new(WorldState::new(), config.clone());
    let mut right = RuntimeKernel::new(WorldState::new(), config);

    left.enqueue_apply_segment(ApplySegment {
        region_key: (1, 0, 0),
        family_tags: vec![4, 2],
    })
    .unwrap();
    left.enqueue_apply_segment(ApplySegment {
        region_key: (0, 0, 0),
        family_tags: vec![3],
    })
    .unwrap();

    right
        .enqueue_apply_segment(ApplySegment {
            region_key: (0, 0, 0),
            family_tags: vec![3],
        })
        .unwrap();
    right
        .enqueue_apply_segment(ApplySegment {
            region_key: (1, 0, 0),
            family_tags: vec![2, 4],
        })
        .unwrap();

    let left_result = left.run_tick().unwrap();
    let right_result = right.run_tick().unwrap();

    assert_eq!(left_result, right_result);
    assert_eq!(
        left.deterministic_digest().unwrap(),
        right.deterministic_digest().unwrap()
    );
}

#[test]
fn runtime_digest_changes_when_apply_input_changes() {
    let config = RuntimeConfig {
        profile: RuntimeProfile::Interactive60,
        max_apply_segments_per_tick: 4,
        publish_passes: 1,
    };
    let mut left = RuntimeKernel::new(WorldState::new(), config.clone());
    let mut right = RuntimeKernel::new(WorldState::new(), config);

    left.enqueue_apply_segment(ApplySegment {
        region_key: (0, 0, 0),
        family_tags: vec![1],
    })
    .unwrap();
    right
        .enqueue_apply_segment(ApplySegment {
            region_key: (0, 0, 0),
            family_tags: vec![2],
        })
        .unwrap();

    left.run_tick().unwrap();
    right.run_tick().unwrap();

    assert_ne!(
        left.deterministic_digest().unwrap(),
        right.deterministic_digest().unwrap()
    );
}
