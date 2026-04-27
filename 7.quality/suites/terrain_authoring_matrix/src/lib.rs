#[cfg(test)]
mod tests {
    use stratumx_tooling_l6_1_command_envelopes::canonical_button_routes;

    fn route(
        button_id: &str,
    ) -> &'static stratumx_tooling_l6_1_command_envelopes::CanonicalButtonRoute {
        canonical_button_routes()
            .iter()
            .find(|route| route.button_id == button_id)
            .expect("terrain route must exist")
    }

    #[test]
    fn terrain_routes_are_owned_by_terrain_or_import_services() {
        let expectations = [
            (
                "btn.import.heightmap_source",
                "ImportExportPipelineService",
                "ShellChanged",
            ),
            (
                "btn.terrain.import_heightmap",
                "ImportExportPipelineService",
                "DiagnosticsChanged",
            ),
            (
                "btn.terrain.sculpt_primary",
                "TerrainAuthoringService",
                "DiagnosticsChanged",
            ),
            (
                "btn.terrain.smooth_patch",
                "TerrainAuthoringService",
                "DiagnosticsChanged",
            ),
            (
                "btn.terrain.flatten_patch",
                "TerrainAuthoringService",
                "DiagnosticsChanged",
            ),
            (
                "btn.terrain.paint_layer",
                "TerrainAuthoringService",
                "DiagnosticsChanged",
            ),
            (
                "btn.terrain.paint_biome_mask",
                "TerrainAuthoringService",
                "DiagnosticsChanged",
            ),
            (
                "btn.terrain.rebuild_chunks",
                "TerrainAuthoringService",
                "DiagnosticsChanged",
            ),
            (
                "btn.terrain.load_chunks",
                "TerrainAuthoringService",
                "DiagnosticsChanged",
            ),
            (
                "btn.terrain.save_chunks",
                "TerrainAuthoringService",
                "DiagnosticsChanged",
            ),
        ];

        for (button_id, owner_service, publication_kind) in expectations {
            let route = route(button_id);
            assert_eq!(route.owner_service, owner_service);
            assert!(route
                .publication_kinds
                .iter()
                .any(|kind| kind == publication_kind));
        }
    }

    #[test]
    fn terrain_mutations_define_persistence_posture() {
        for button_id in [
            "btn.terrain.import_heightmap",
            "btn.terrain.sculpt_primary",
            "btn.terrain.smooth_patch",
            "btn.terrain.flatten_patch",
            "btn.terrain.paint_layer",
            "btn.terrain.paint_biome_mask",
            "btn.terrain.rebuild_chunks",
            "btn.terrain.save_chunks",
        ] {
            let route = route(button_id);
            assert_ne!(route.persistence_kind, "none");
            assert!(!route.recovery_anchor.is_empty());
        }
    }
}
