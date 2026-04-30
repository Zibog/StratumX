pub(crate) const ROUTE_PROJECT_BOOTSTRAP: &str = "route.project.bootstrap.v1";
pub(crate) const ROUTE_PROJECT_CREATE: &str = "route.project.create.v1";
pub(crate) const ROUTE_PROJECT_SAVE: &str = "route.project.save.v1";
pub(crate) const ROUTE_BUILD_PACKAGE: &str = "route.build.package.v1";
pub(crate) const ROUTE_EXPORT_TARGET: &str = "route.export.target.v1";
pub(crate) const ROUTE_LAUNCH_VERIFY_FIRST_RESULT: &str = "route.launch.verify_first_result.v1";
pub(crate) const ROUTE_WORLD_OPEN: &str = "route.world.open.v1";
pub(crate) const ROUTE_WORLD_SAVE: &str = "route.world.save.v1";
pub(crate) const ROUTE_WORLD_CLOSE: &str = "route.world.close.v1";
pub(crate) const ROUTE_RUNTIME_PLAY: &str = "route.runtime.play.v1";
pub(crate) const ROUTE_RUNTIME_PAUSE: &str = "route.runtime.pause.v1";
pub(crate) const ROUTE_RUNTIME_STOP: &str = "route.runtime.stop.v1";
pub(crate) const ROUTE_RUNTIME_SIMULATE: &str = "route.runtime.simulate.v1";
pub(crate) const ROUTE_TERRAIN_IMPORT: &str = "route.terrain.import.v1";
pub(crate) const ROUTE_TERRAIN_REBUILD: &str = "route.terrain.rebuild.v1";
pub(crate) const ROUTE_TERRAIN_SCULPT_RAISE: &str = "route.terrain.sculpt.raise.v1";
pub(crate) const ROUTE_TERRAIN_SCULPT_LOWER: &str = "route.terrain.sculpt.lower.v1";
pub(crate) const ROUTE_TERRAIN_SMOOTH_PATCH: &str = "route.terrain.smooth_patch.v1";
pub(crate) const ROUTE_TERRAIN_SCULPT_FLATTEN: &str = "route.terrain.sculpt.flatten.v1";
pub(crate) const ROUTE_TERRAIN_PAINT_MATERIAL: &str = "route.terrain.paint.material.v1";
pub(crate) const ROUTE_TERRAIN_LAYER_MATERIAL_SET: &str = "route.terrain.layer.material.set.v1";
pub(crate) const ROUTE_TERRAIN_HOLE_ADD: &str = "route.terrain.hole.add.v1";
pub(crate) const ROUTE_TERRAIN_HOLE_REMOVE: &str = "route.terrain.hole.remove.v1";
pub(crate) const ROUTE_SKY_SET_TIME_OF_DAY: &str = "route.sky.set_time_of_day.v1";
pub(crate) const ROUTE_SKY_SET_WEATHER_REGIME: &str = "route.sky.set_weather_regime.v1";
pub(crate) const ROUTE_SKY_BIND_CLOUD_PROFILE: &str = "route.sky.bind_cloud_profile.v1";
pub(crate) const ROUTE_ENVIRONMENT_FOG_DENSITY: &str = "route.environment.fog.density.v1";
pub(crate) const ROUTE_SKY_BIND_PROFILE: &str = "route.sky.bind_profile.v1";
pub(crate) const ROUTE_SHELL_ACTIVATE_VIEWPORT: &str = "route.shell.activate_viewport.v1";
pub(crate) const ROUTE_SHELL_ACTIVATE_OUTLINER: &str = "route.shell.activate_outliner.v1";
pub(crate) const ROUTE_SHELL_ACTIVATE_INSPECTOR: &str = "route.shell.activate_inspector.v1";
pub(crate) const ROUTE_SHELL_ACTIVATE_CONTENT_BROWSER: &str =
    "route.shell.activate_content_browser.v1";
pub(crate) const ROUTE_SHELL_ACTIVATE_MATERIAL_SURFACE: &str =
    "route.shell.activate_material_surface.v1";
pub(crate) const ROUTE_SHELL_ACTIVATE_TERRAIN_SURFACE: &str =
    "route.shell.activate_terrain_surface.v1";
pub(crate) const ROUTE_SHELL_ACTIVATE_SKY_SURFACE: &str = "route.shell.activate_sky_surface.v1";
pub(crate) const ROUTE_MATERIAL_AUTHORITY_INITIALIZE: &str =
    "route.material.authority.initialize.v1";
pub(crate) const ROUTE_MATERIAL_AUTHORITY_DISPOSE: &str = "route.material.authority.dispose.v1";
pub(crate) const ROUTE_AUDIO_AUTHORITY_INITIALIZE: &str = "route.audio.authority.initialize.v1";
pub(crate) const ROUTE_AUDIO_AUTHORITY_DISPOSE: &str = "route.audio.authority.dispose.v1";
pub(crate) const ROUTE_MATERIAL_CREATE: &str = "route.material.create.v1";
pub(crate) const ROUTE_MATERIAL_DELETE: &str = "route.material.delete.v1";
pub(crate) const ROUTE_MATERIAL_DUPLICATE_PROFILE: &str = "route.material.duplicate_profile.v1";
pub(crate) const ROUTE_MATERIAL_BIND_VISUAL_RESPONSE: &str =
    "route.material.bind_visual_response.v1";
pub(crate) const ROUTE_MATERIAL_BIND_ACOUSTIC_PROFILE: &str =
    "route.material.bind_acoustic_profile.v1";
pub(crate) const ROUTE_MATERIAL_BIND_LIGHT_RESPONSE: &str = "route.material.bind_light_response.v1";
pub(crate) const ROUTE_MATERIAL_BIND_MICRODETAIL_PROFILE: &str =
    "route.material.bind_microdetail_profile.v1";
pub(crate) const ROUTE_MATERIAL_BIND_WEATHER_MODULATION: &str =
    "route.material.bind_weather_modulation.v1";
pub(crate) const ROUTE_MATERIAL_PREVIEW_BURN: &str = "route.material.preview_burn.v1";
pub(crate) const ROUTE_MATERIAL_SET_CHEAP_RUNTIME_RUNG: &str =
    "route.material.set_cheap_runtime_rung.v1";
pub(crate) const ROUTE_MATERIAL_INSPECT_COVERAGE: &str = "route.material.inspect_coverage.v1";
pub(crate) const ROUTE_AUDIO_SOURCE_CREATE: &str = "route.audio.source.create.v1";
pub(crate) const ROUTE_AUDIO_SOURCE_BIND: &str = "route.audio.source.bind.v1";
pub(crate) const ROUTE_AUDIO_ACOUSTIC_SET: &str = "route.audio.acoustic.set.v1";
pub(crate) const ROUTE_AUDIO_ASSIGN_EMITTER_CLASS_WORLD_SOURCE: &str =
    "route.audio.assign_emitter_class_world_source.v1";
pub(crate) const ROUTE_AUDIO_BIND_ZONE_PROFILE_WORLD_SURFACE: &str =
    "route.audio.bind_zone_profile_world_surface.v1";
pub(crate) const ROUTE_AUDIO_BIND_PRIORITY_DUCKING_POLICY: &str =
    "route.audio.bind_priority_ducking_policy.v1";
pub(crate) const ROUTE_AUDIO_PREVIEW_AUDIBILITY_FREE_CAMERA: &str =
    "route.audio.preview_audibility_free_camera.v1";
pub(crate) const ROUTE_AUDIO_PREVIEW_OBSTRUCTION_VS_OCCLUSION: &str =
    "route.audio.preview_obstruction_vs_occlusion.v1";
pub(crate) const ROUTE_AUDIO_PREVIEW_INDOOR_OUTDOOR_TRANSITION: &str =
    "route.audio.preview_indoor_outdoor_transition.v1";
pub(crate) const ROUTE_AUDIO_PREVIEW_VOICE_SUBTITLE_LEGALITY: &str =
    "route.audio.preview_voice_subtitle_legality.v1";
pub(crate) const ROUTE_BUILD_RUN: &str = "route.build.run.v1";
pub(crate) const ROUTE_BUILD_RELEASE: &str = "route.build.release.v1";
pub(crate) const ROUTE_WORLD_VALIDATE: &str = "route.world.validate.v1";
pub(crate) const ROUTE_VALIDATION_SMOKE: &str = "route.validation.smoke.v1";
pub(crate) const ROUTE_AUTOMATION_REBUILD_ALL: &str = "route.automation.rebuild_all.v1";
pub(crate) const ROUTE_AUTOMATION_VALIDATE_ALL: &str = "route.automation.validate_all.v1";
pub(crate) const ROUTE_SCENE_BOOTSTRAP: &str = "route.scene.bootstrap.v1";
pub(crate) const ROUTE_SCENE_FIRE_TEST_SHOT: &str = "route.scene.fire_test_shot.v1";
pub(crate) const ROUTE_SCENE_RESET: &str = "route.scene.reset.v1";
