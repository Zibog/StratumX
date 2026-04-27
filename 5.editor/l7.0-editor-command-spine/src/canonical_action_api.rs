use stratumx_tooling_l6_1_command_envelopes::{
    canonical_route_by_action_id, CanonicalCommandEnvelope, CommandPayload, SourceSurface,
};

macro_rules! canonical_actions {
    ($($variant:ident => ($action:expr, $button:expr)),+ $(,)?) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)] pub enum CanonicalActionId { $($variant),+ }
        impl CanonicalActionId {
            pub fn as_str(&self) -> &'static str { match self { $(Self::$variant => $action),+ } }
            pub fn button_id(&self) -> &'static str { match self { $(Self::$variant => $button),+ } }
            pub const fn all() -> &'static [Self] { &[$(Self::$variant),+] }
            pub fn from_action_id(value: &str) -> Option<Self> { match value { $($action => Some(Self::$variant)),+, _ => None } }
            pub fn from_button_id(value: &str) -> Option<Self> { match value { $($button => Some(Self::$variant)),+, _ => None } }
        }
    };
}

canonical_actions! {
    ProjectNewProject => ("project.new_project.requested", "btn.project.new_project"), ProjectOpenProject => ("project.open_project.requested", "btn.project.open_project"), ProjectSaveProject => ("project.save_project.requested", "btn.project.save_project"), ProjectSaveProjectAs => ("project.save_project_as.requested", "btn.project.save_project_as"), WorldOpenWorldPackage => ("world.open_world_package.requested", "btn.world.open_world_package"), WorldSaveWorldPackage => ("world.save_world_package.requested", "btn.world.save_world_package"), WorldValidateWorld => ("world.validate_world.requested", "btn.world.validate_world"), ImportHeightmapSource => ("import.heightmap_source.selected", "btn.import.heightmap_source"), TerrainImportHeightmap => ("terrain.import_heightmap.requested", "btn.terrain.import_heightmap"), TerrainSculptPrimary => ("terrain.sculpt_primary.requested", "btn.terrain.sculpt_primary"), TerrainSmoothPatch => ("terrain.smooth_patch.requested", "btn.terrain.smooth_patch"), TerrainFlattenPatch => ("terrain.flatten_patch.requested", "btn.terrain.flatten_patch"), TerrainPaintLayer => ("terrain.paint_layer.requested", "btn.terrain.paint_layer"), TerrainPaintBiomeMask => ("terrain.paint_biome_mask.requested", "btn.terrain.paint_biome_mask"), TerrainRebuildChunks => ("terrain.rebuild_chunks.requested", "btn.terrain.rebuild_chunks"), TerrainLoadChunks => ("terrain.load_chunks.requested", "btn.terrain.load_chunks"), TerrainSaveChunks => ("terrain.save_chunks.requested", "btn.terrain.save_chunks"), MaterialNewProfile => ("material.new_profile.requested", "btn.material.new_profile"), MaterialDuplicateProfile => ("material.duplicate_profile.requested", "btn.material.duplicate_profile"), MaterialAssignArchetype => ("material.assign_archetype.requested", "btn.material.assign_archetype"), MaterialBindSurfaceFamily => ("material.bind_surface_family.requested", "btn.material.bind_surface_family"), MaterialBindResponseProfile => ("material.bind_response_profile.requested", "btn.material.bind_response_profile"), MaterialBindTextureStack => ("material.bind_texture_stack.requested", "btn.material.bind_texture_stack"), MaterialBindMicrodetailProfile => ("material.bind_microdetail_profile.requested", "btn.material.bind_microdetail_profile"), MaterialBindWeatherModulation => ("material.bind_weather_modulation.requested", "btn.material.bind_weather_modulation"), MaterialBindVisualResponseFamily => ("material.bind_visual_response_family.requested", "btn.material.bind_visual_response_family"), MaterialBindAcousticProfile => ("material.bind_acoustic_profile.requested", "btn.material.bind_acoustic_profile"), MaterialBindLightResponse => ("material.bind_light_response.requested", "btn.material.bind_light_response"), MaterialInspectBranchCoverage => ("material.inspect_branch_coverage.requested", "btn.material.inspect_branch_coverage"), MaterialSetCheapRuntimeRung => ("material.set_cheap_runtime_rung.requested", "btn.material.set_cheap_runtime_rung"), MaterialPreviewBulletHit => ("material.preview_bullet_hit.requested", "btn.material.preview_bullet_hit"), MaterialPreviewBlast => ("material.preview_blast.requested", "btn.material.preview_blast"), MaterialPreviewWetness => ("material.preview_wetness.requested", "btn.material.preview_wetness"), MaterialPreviewBurn => ("material.preview_burn.requested", "btn.material.preview_burn"), MaterialSaveProfileAs => ("material.save_profile_as.requested", "btn.material.save_profile_as"), MaterialCaptureProofArtifacts => ("material.capture_proof_artifacts.requested", "btn.material.capture_proof_artifacts"), MaterialReviewFreezeBlockers => ("material.review_freeze_blockers.requested", "btn.material.review_freeze_blockers"), SkyBindSkyProfile => ("sky.bind_sky_profile.requested", "btn.sky.bind_sky_profile"), SkySetTimeOfDay => ("sky.set_time_of_day.requested", "btn.sky.set_time_of_day"), SkySetWeatherRegime => ("sky.set_weather_regime.requested", "btn.sky.set_weather_regime"), SkyBindCloudProfile => ("sky.bind_cloud_profile.requested", "btn.sky.bind_cloud_profile"), ViewViewport => ("view.viewport.requested", "btn.view.viewport"), ViewOutliner => ("view.outliner.requested", "btn.view.outliner"), ViewInspector => ("view.inspector.requested", "btn.view.inspector"), ViewContentBrowser => ("view.content_browser.requested", "btn.view.content_browser"), ViewMaterialLab => ("view.material_lab.requested", "btn.view.material_lab"), ViewTerrainLab => ("view.terrain_lab.requested", "btn.view.terrain_lab"), ViewSkyLab => ("view.sky_lab.requested", "btn.view.sky_lab"), AudioAssignEmitterClassWorldSource => ("audio.assign_emitter_class_world_source.requested", "btn.audio.assign_emitter_class_world_source"), AudioBindZoneProfileWorldSurface => ("audio.bind_zone_profile_world_surface.requested", "btn.audio.bind_zone_profile_world_surface"), AudioBindPriorityDuckingPolicy => ("audio.bind_priority_ducking_policy.requested", "btn.audio.bind_priority_ducking_policy"), AudioPreviewAudibilityFreeCamera => ("audio.preview_audibility_free_camera.requested", "btn.audio.preview_audibility_free_camera"), AudioPreviewObstructionVsOcclusion => ("audio.preview_obstruction_vs_occlusion.requested", "btn.audio.preview_obstruction_vs_occlusion"), AudioPreviewIndoorOutdoorTransition => ("audio.preview_indoor_outdoor_transition.requested", "btn.audio.preview_indoor_outdoor_transition"), AudioPreviewVoiceSubtitleLegality => ("audio.preview_voice_subtitle_legality.requested", "btn.audio.preview_voice_subtitle_legality"), AudioInspectListenerProfile => ("audio.inspect_listener_profile.requested", "btn.audio.inspect_listener_profile"), AudioInspectBusDucking => ("audio.inspect_bus_ducking.requested", "btn.audio.inspect_bus_ducking")
}

#[derive(Debug, Clone, PartialEq)]
pub struct ActionRequest<P> {
    pub action_id: CanonicalActionId,
    pub payload: P,
    pub ui_context: UiActionContext,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UiActionContext {
    pub source_surface: SourceSurface,
    pub has_project: bool,
    pub has_world: bool,
    pub has_selection: bool,
    pub shell_ready: bool,
}
impl UiActionContext {
    pub fn shell_only(source_surface: SourceSurface) -> Self {
        Self {
            source_surface,
            has_project: false,
            has_world: false,
            has_selection: false,
            shell_ready: true,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ActionDenial {
    ShellUnavailable,
    MissingProject,
    MissingWorld,
    MissingSelection,
    RouteNotCataloged(String),
}

pub struct CanonicalActionDispatch;
impl CanonicalActionDispatch {
    pub fn dispatch<P: Into<CommandPayload>>(
        request: ActionRequest<P>,
    ) -> Result<CanonicalCommandEnvelope<CommandPayload>, ActionDenial> {
        Self::validate(&request.action_id, &request.ui_context)?;
        let route = canonical_route_by_action_id(request.action_id.as_str()).ok_or_else(|| {
            ActionDenial::RouteNotCataloged(request.action_id.as_str().to_string())
        })?;
        Ok(CanonicalCommandEnvelope::from_route(
            route,
            request.payload.into(),
            request.ui_context.source_surface,
        ))
    }
    fn validate(
        action_id: &CanonicalActionId,
        ui_context: &UiActionContext,
    ) -> Result<(), ActionDenial> {
        if !ui_context.shell_ready {
            return Err(ActionDenial::ShellUnavailable);
        }
        let button_id = action_id.button_id();
        let is_view_route = button_id.starts_with("btn.view.");
        let needs_project = !is_view_route && !button_id.starts_with("btn.project.new_project");
        let needs_world = button_id.starts_with("btn.world.save_")
            || button_id == "btn.world.validate_world"
            || button_id.starts_with("btn.terrain.")
            || button_id.starts_with("btn.material.")
            || button_id.starts_with("btn.sky.")
            || button_id.starts_with("btn.audio.");
        if needs_project && !ui_context.has_project {
            return Err(ActionDenial::MissingProject);
        }
        if needs_world && !ui_context.has_world {
            return Err(ActionDenial::MissingWorld);
        }
        if matches!(
            action_id,
            CanonicalActionId::MaterialBindSurfaceFamily
                | CanonicalActionId::MaterialBindResponseProfile
                | CanonicalActionId::MaterialBindTextureStack
                | CanonicalActionId::MaterialBindMicrodetailProfile
                | CanonicalActionId::MaterialBindWeatherModulation
                | CanonicalActionId::MaterialBindVisualResponseFamily
                | CanonicalActionId::MaterialBindAcousticProfile
                | CanonicalActionId::MaterialBindLightResponse
                | CanonicalActionId::AudioAssignEmitterClassWorldSource
                | CanonicalActionId::AudioBindZoneProfileWorldSurface
        ) && !ui_context.has_selection
        {
            return Err(ActionDenial::MissingSelection);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use stratumx_tooling_l6_1_command_envelopes::MaterialPayload;
    #[test]
    fn canonical_actions_lower_to_envelopes() {
        let envelope = CanonicalActionDispatch::dispatch(ActionRequest {
            action_id: CanonicalActionId::MaterialBindLightResponse,
            payload: MaterialPayload::default(),
            ui_context: UiActionContext {
                source_surface: SourceSurface::MaterialPanel,
                has_project: true,
                has_world: true,
                has_selection: true,
                shell_ready: true,
            },
        })
        .expect("canonical dispatch");
        assert_eq!(
            envelope.route_metadata.button_id,
            "btn.material.bind_light_response"
        );
    }
    #[test]
    fn action_catalog_round_trips_from_string_ids() {
        let action = CanonicalActionId::from_action_id("material.bind_light_response.requested")
            .expect("action id lookup");
        assert_eq!(action, CanonicalActionId::MaterialBindLightResponse);
        assert_eq!(
            CanonicalActionId::from_button_id("btn.material.bind_light_response"),
            Some(CanonicalActionId::MaterialBindLightResponse)
        );
        assert_eq!(CanonicalActionId::all().len(), 57);
    }
}
