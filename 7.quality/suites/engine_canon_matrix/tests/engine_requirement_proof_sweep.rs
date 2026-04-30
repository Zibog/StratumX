use std::fs;
use std::path::PathBuf;

fn suite_source(stem: &str) -> String {
    let tests_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests");
    let root = tests_dir.join(format!("{stem}.rs"));
    let mut combined = fs::read_to_string(&root).unwrap_or_else(|error| {
        panic!(
            "failed to read test suite source {}: {error}",
            root.display()
        )
    });

    let chunk_dir = tests_dir.join(stem);
    if chunk_dir.is_dir() {
        let mut chunks = chunk_dir
            .read_dir()
            .unwrap_or_else(|error| {
                panic!("failed to read chunk dir {}: {error}", chunk_dir.display())
            })
            .map(|entry| entry.unwrap().path())
            .filter(|path| path.extension().and_then(|ext| ext.to_str()) == Some("rs"))
            .collect::<Vec<_>>();
        chunks.sort();
        for chunk in chunks {
            combined.push('\n');
            combined.push_str(&fs::read_to_string(&chunk).unwrap_or_else(|error| {
                panic!("failed to read chunk source {}: {error}", chunk.display())
            }));
        }
    }

    combined
}

#[test]
fn phase_1_storage_mutation_tests_exist() {
    let storage_authoritative_apply = suite_source("storage_mutation_authoritative_apply");
    let storage_law = suite_source("storage_mutation_law");

    assert!(storage_authoritative_apply.contains("successful_authoritative_apply_mutates_target"));
    assert!(storage_authoritative_apply.contains("failed_validation_does_not_mutate_target"));
    assert!(storage_authoritative_apply.contains("conflicting_non_idempotent_write_rejected"));
    assert!(storage_authoritative_apply.contains("idempotent_replay_is_stable"));
    assert!(storage_authoritative_apply.contains("non_idempotent_replay_without_guard_rejected"));
    assert!(storage_authoritative_apply.contains("dry_run_does_not_mutate"));
    assert!(storage_law.contains("partial_failure_does_not_report_fake_success"));
}

#[test]
fn phase_2_runtime_profile_tests_exist() {
    let memory_control_law = suite_source("memory_control");
    let runtime_profile_law = suite_source("runtime_profile_law");
    let runtime_memory_law = suite_source("runtime_profiles_memory_law");

    assert!(runtime_profile_law.contains("headless_runtime_rejects_invalid_state"));
    assert!(runtime_profile_law.contains("realtime_zero_fps_rejected"));
    assert!(
        runtime_memory_law.contains("headless_snapshot_receipt_digest_is_stable_for_same_state")
    );
    assert!(runtime_memory_law.contains("headless_replay_digest_changes_when_state_changes"));
    assert!(runtime_memory_law.contains("realtime_frame_receipt_contains_monotonic_frame_id"));
    assert!(
        memory_control_law.contains("release_path_rejects_wrong_size_unknown_and_double_release")
    );
}

#[test]
fn phase_3_material_burn_path_tests_exist() {
    let material_burn_fire_law = suite_source("material_burn_fire_law");

    assert!(
        material_burn_fire_law.contains("fire_exposure_runtime_emits_receipt_and_material_event")
    );
    assert!(material_burn_fire_law
        .contains("wetness_changes_ignition_branch_without_bypassing_runtime_path"));
    assert!(material_burn_fire_law.contains("invalid_fire_exposure_profile_is_rejected"));
    assert!(material_burn_fire_law.contains("same_fire_exposure_input_produces_same_digest"));
}

#[test]
fn phase_4_network_boundary_tests_exist() {
    let network_runtime_law = suite_source("network_runtime_services_law");

    assert!(network_runtime_law.contains("stale_session_epoch_fails"));
    assert!(network_runtime_law.contains("prediction_outside_rewind_window_fails"));
    assert!(network_runtime_law.contains("send_without_open_session_fails"));
    assert!(network_runtime_law.contains("delta_without_base_snapshot_fails"));
    assert!(network_runtime_law.contains("reconcile_emits_authoritative_correction_receipt"));
}

#[test]
fn phase_5_upper_engine_boundary_tests_exist() {
    let content_boundary_law = suite_source("content_runtime_pack_law");
    let imaging_boundary_law = suite_source("imaging_boundary_law");
    let acoustics_boundary_law = suite_source("acoustics_boundary_law");
    let model_boundary_law = suite_source("model_boundary_law");
    let animation_boundary_law = suite_source("animation_boundary_law");
    let upper_engine_boundary_law = suite_source("upper_engine_boundary_law");

    assert!(content_boundary_law.contains("invalid_locator_is_rejected_with_exact_reason"));
    assert!(
        content_boundary_law.contains("different_dependency_graph_changes_deterministic_digest")
    );
    assert!(imaging_boundary_law.contains("imaging_missing_material_uses_explicit_fallback_policy"));
    assert!(acoustics_boundary_law.contains("acoustics_missing_material_uses_explicit_fallback"));
    assert!(model_boundary_law.contains("inference_missing_model_is_rejected_with_exact_reason"));
    assert!(
        animation_boundary_law.contains("animation_invalid_joint_count_fails_with_exact_reason")
    );
    assert!(upper_engine_boundary_law.contains("content_runtime_pack_product_deterministic"));
}

#[test]
fn all_critical_blockers_have_test_coverage() {
    let suites = [
        suite_source("storage_mutation_authoritative_apply"),
        suite_source("storage_mutation_law"),
        suite_source("memory_control"),
        suite_source("runtime_profile_law"),
        suite_source("runtime_profiles_memory_law"),
        suite_source("material_burn_fire_law"),
        suite_source("network_boundary_law"),
        suite_source("network_runtime_services_law"),
        suite_source("content_runtime_pack_law"),
        suite_source("imaging_boundary_law"),
        suite_source("acoustics_boundary_law"),
        suite_source("model_boundary_law"),
        suite_source("animation_boundary_law"),
        suite_source("upper_engine_boundary_law"),
    ];

    let total_tests: usize = suites
        .iter()
        .map(|suite| suite.matches("#[test]").count())
        .sum();
    assert!(
        total_tests >= 65,
        "expected at least 65 blocker-law tests, found {total_tests}"
    );
}

#[test]
fn proof_requirements_met() {
    let suites = [
        suite_source("storage_mutation_authoritative_apply"),
        suite_source("storage_mutation_law"),
        suite_source("memory_control"),
        suite_source("runtime_profile_law"),
        suite_source("runtime_profiles_memory_law"),
        suite_source("material_burn_fire_law"),
        suite_source("network_boundary_law"),
        suite_source("network_runtime_services_law"),
        suite_source("content_runtime_pack_law"),
        suite_source("imaging_boundary_law"),
        suite_source("acoustics_boundary_law"),
        suite_source("model_boundary_law"),
        suite_source("animation_boundary_law"),
        suite_source("upper_engine_boundary_law"),
    ];

    for suite in suites {
        assert!(!suite.contains("#[ignore]"));
        assert!(!suite.contains("assert!(true"));
        assert!(!suite.contains("u64 >= 0"));
        assert!(!suite.contains("digest exists"));
    }
}
