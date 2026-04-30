#[test]
fn presentation_budget_decision_tracks_deterministic_budget() {
    let world = WorldState::new();
    let config = RealtimeRuntimeConfig {
        target_fps: 60,
        visibility_freshness_frames: 2,
        enqueue_presentable_frames: false,
    };
    let profile = RealtimeRuntimeProfile::new(world, config).unwrap();

    let cadence = profile.cadence().unwrap();
    assert_eq!(
        profile
            .presentation_budget_decision(cadence.frame_budget_micros - 1)
            .unwrap(),
        PresentationBudgetDecision::Present
    );
    assert_eq!(
        profile
            .presentation_budget_decision(cadence.frame_budget_micros + 1)
            .unwrap(),
        PresentationBudgetDecision::Degrade
    );
    assert_eq!(
        profile
            .presentation_budget_decision(cadence.frame_budget_micros.saturating_mul(3))
            .unwrap(),
        PresentationBudgetDecision::Skip
    );
}
