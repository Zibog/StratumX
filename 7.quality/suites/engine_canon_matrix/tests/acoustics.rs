//! Acoustics: проверка звуковой системы

mod common;
use common::*;
use engine_acoustics::{AudioRuntime, AudioSource, FootstepEvent, FootstepMaterial, Occluder};

#[test]
fn sound_propagation() {
    let mut transfer = TransferControlService::new(TransferConfig {
        max_inflight_decodes: 4,
        max_inflight_uploads: 4,
    });
    let service = AcousticsService::new(AcousticsConfig {
        max_sources: 8,
        max_stream_upload_bytes: 128,
    });

    let result = service
        .synthesize(
            &WorldState::new(),
            &EcsSubstrate::new(),
            &materials(),
            &ResidencyControlService::new(ResidencyConfig {
                resident_item_budget: 4,
                streaming_item_budget: 4,
            }),
            &mut transfer,
            AcousticsRequest {
                source_count: 2,
                stream_upload_bytes: 24,
            },
        )
        .unwrap();

    let upload = transfer.complete_upload().unwrap();
    assert_eq!(result.propagated_sources, 2);
    assert_eq!(result.synthesized_frames, 1);
    assert_eq!(upload.upload_bytes, 24);
}

#[test]
fn sound_occlusion() {
    let mut open_air = AudioRuntime::new();
    open_air.add_source(AudioSource::gunshot([0.0, 0.0, 10.0]));
    open_air.set_listener_position([0.0, 0.0, 0.0]);
    let unobstructed_mix = open_air.calculate_mix();

    let mut occluded = open_air.clone();
    occluded.add_occluder(Occluder {
        position: [0.0, 0.0, 5.0],
        bounds: [1.0, 1.0, 1.0],
        occlusion_factor: 0.5,
    });

    assert!(occluded.calculate_mix() < unobstructed_mix);
}

#[test]
fn sound_reverb() {
    let mut runtime = AudioRuntime::new();
    runtime.add_footstep(FootstepEvent {
        position: [1.0, 0.0, 0.0],
        material: FootstepMaterial::Metal,
        velocity: 5.0,
    });

    assert_eq!(runtime.sources.len(), 1);
    assert_eq!(
        runtime.sources[0].sound_id,
        FootstepMaterial::Metal.sound_id()
    );

    runtime.update(1.0);
    assert!(runtime.sources.is_empty());
}
