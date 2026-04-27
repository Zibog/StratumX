use proptest::prelude::*;

use stratumx_editor_l9_3_material_lookdev_authoring_suite::{
    MaterialAuthoringService, RuntimeCheapnessRung, TextureSlot,
};

fn slot_strategy() -> impl Strategy<Value = TextureSlot> {
    prop_oneof![
        Just(TextureSlot::Albedo),
        Just(TextureSlot::Normal),
        Just(TextureSlot::Roughness),
        Just(TextureSlot::Metallic),
    ]
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(64))]

    #[test]
    fn prop_cheapness_report_tracks_texture_count(
        slots in prop::collection::vec(slot_strategy(), 0..4),
        configure_rung in any::<bool>(),
    ) {
        let mut service = MaterialAuthoringService::new();
        let handle = service.create_material("PropMaterial".to_string());

        if configure_rung {
            service
                .set_cheap_runtime_rung(handle, RuntimeCheapnessRung::CachedLocal)
                .expect("set rung");
        }

        for slot in slots.iter().copied() {
            let path = match slot {
                TextureSlot::Albedo => "albedo.png",
                TextureSlot::Normal => "normal.png",
                TextureSlot::Roughness => "roughness.png",
                TextureSlot::Metallic => "metallic.png",
            };
            service
                .assign_texture(handle, slot, path.to_string())
                .expect("assign texture");
        }

        let report = service.validate_cheapness(handle).expect("cheapness report");
        let unique_slots = slots.into_iter().collect::<std::collections::BTreeSet<_>>().len();

        prop_assert_eq!(report.texture_count, unique_slots);
        if !configure_rung {
            prop_assert!(!report.passes);
        }
    }
}
