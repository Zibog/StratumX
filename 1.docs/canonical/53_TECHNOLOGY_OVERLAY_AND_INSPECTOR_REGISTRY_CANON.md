# Technology Overlay and Inspector Registry Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

| Family | Required overlays | Required inspector fields |
|---|---|---|
| frame | pass ownership, culling class, presentation target | `viewset_ref`, `pass_graph_ref`, `presentation_target`, `frame_result_code` |
| residency | residency bucket, streaming pressure, mip fallback | `residency_bucket`, `requested_mip`, `resolved_mip`, `stream_queue_depth` |
| lighting | light slice, shadow class, atmosphere class | `light_slice_id`, `shadow_budget_code`, `atmosphere_profile`, `exposure_mode` |
| audio | route path, zone/occlusion, device fallback | `route_id`, `zone_id`, `occlusion_class`, `device_profile`, `mix_profile` |
| diagnostics | boundary owner, failure code, next action | `trace_id`, `owner_package`, `failure_code`, `next_action_id` |
| resource | pressure vector, threshold code, degrade step | `resource_vector`, `threshold_code`, `degrade_step`, `recovery_trigger` |
| material truth | material family overlay, response profile overlay, wetness overlay, aftermath overlay | `material_archetype_ref`, `surface_family_ref`, `response_profile_ref`, `weather_modulation_profile_ref`, `aftermath_overlay_profile_ref` |
| terrain meaning | terrain material weights, biome overlay heatmap, terrain layer weights, sleeping vs woken material state | `terrain_layer_weight_map_ref`, `biome_overlay_map_ref`, `consequence_tier_ref`, `sleep_wake_policy_ref`, `chunk_region_ref` |
| world context | cloud profile, time-of-day context, weather regime, consequence-tier visibility | `world_ref`, `sky_profile_ref`, `cloud_profile_ref`, `weather_regime_ref`, `consequence_tier_ref` |


## Base-shell promoted overlay and inspector additions
| Family | Required overlays | Required inspector fields |
|---|---|---|
| shell/view | active surface signal, selection preservation hint, unavailable-surface marker | `active_surface_id`, `previous_surface_id`, `selection_ref`, `surface_availability_code` |
| sky authoring | time-of-day context, weather regime, cloud binding, horizon visibility | `sky_profile_ref`, `time_of_day`, `weather_regime_ref`, `cloud_profile_ref`, `horizon_visibility_code` |
| material presentation | microdetail profile, weather modulation, burn preview state | `microdetail_profile_ref`, `weather_modulation_profile_ref`, `preview_burn_state`, `material_stack_completeness_code` |
| world validation | world blocker overlay, degraded-posture badge, chunk integrity status | `world_validation_verdict`, `degraded_posture_code`, `chunk_integrity_code`, `material_linkage_code` |
| audio world authoring | audibility hints, obstruction-vs-occlusion comparison, indoor/outdoor transition, zone binding | `emitter_class_ref`, `zone_profile_ref`, `ducking_policy_ref`, `listener_profile_ref`, `audibility_verdict_code` |

## Material-centric overlays
Required overlays and inspector families now include:
- material response family overlay
- light transmission class overlay
- shadow breakup overlay
- acoustic family / hollowness overlay
- cheap-runtime rung overlay
- missing material branch coverage overlay

## Workspace and viewport-shell overlay additions
| Family | Required overlays | Required inspector fields |
|---|---|---|
| workspace shell | active stage strip, layout state, detached-window count, focused surface chain | `workspace_stage_id`, `layout_id`, `detached_window_count`, `focused_surface_id`, `layout_dirty_flag` |
| viewport shell | render backend badge, present path badge, split-view topology, throttled-view badge | `backend_class`, `shader_target_set`, `present_path_id`, `viewport_topology`, `throttle_code` |
| extension and assistant | mounted-extension badges, capability-grant markers, proposal scope, apply/revert readiness | `extension_bundle_id`, `mount_scope_id`, `capability_grant_code`, `proposal_id`, `rollback_anchor_id` |


## Dream-stack deep overlays and inspector additions
| Family | Required overlays | Required inspector fields |
|---|---|---|
| geodesy / distant world | precision-zone overlay, rebase-delta trail, far-phenomenon tracks | `geo_anchor_ref`, `region_frame_ref`, `cell_frame_ref`, `precision_zone_code`, `far_phenomenon_track_ref` |
| field substrate | wetness/heat/smoke/toxic/wind family heatmaps and volume slices | `field_family_id`, `field_scope_class`, `field_resolution_code`, `field_conflict_verdict`, `field_retention_horizon` |
| destruction topology | support groups, load paths, structural breach classes | `topology_ref`, `support_group_ids`, `load_path_ref`, `breach_class`, `collapse_eligibility_code` |
| terrain deformation | crater/gouge/trench class, removed/deposited mass, nav-cover consequence | `deformation_class`, `dirty_volume_ref`, `removed_mass`, `deposited_mass`, `nav_cover_delta_ref` |
| hydrology | fill ratio, leak geometry, contamination/freeze state, overflow path | `fluid_inventory_ref`, `fill_ratio`, `leak_geometry_ref`, `contamination_class`, `freeze_state` |
| climate theater | front lifecycle, arrival window, lightning tracks, lunar phase | `front_ref`, `front_lifecycle_state`, `arrival_window`, `lightning_event_refs`, `lunar_phase_code` |
| society/tactics/causality | need/status deltas, squad intent, cover validity, why-chain | `need_vector_ref`, `status_delta_ref`, `shared_intent_code`, `cover_validity_digest_ref`, `cause_chain_refs` |
| wound/species | layer stack, organ zone, survivability, gore legality | `species_topology_class`, `layer_entry_ref`, `organ_zone_ref`, `survivability_verdict`, `gore_legality_code` |
| fur/hair/photoreal | representation rung, silhouette/shadow rung, old-floor fallback badges | `representation_rung`, `coverage_mask_ref`, `lighting_rung`, `shadow_rung`, `first_blocker_code` |
