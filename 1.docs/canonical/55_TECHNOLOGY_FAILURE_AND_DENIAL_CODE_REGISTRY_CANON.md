# Technology Failure And Denial Code Registry Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
This registry freezes canonical failure and denial families so that editor, tooling, sdk, and engine speak one code language.

| Family | Canonical codes |
|---|---|
| ownership | `fail.owner_missing`, `fail.scope_forbidden`, `fail.route_truth_mismatch` |
| schema | `fail.schema_revision_mismatch`, `fail.field_missing`, `fail.packet_family_unknown` |
| compare/capture | `fail.baseline_missing`, `fail.capture_target_missing`, `fail.compare_mode_illegal` |
| transaction | `fail.retry_budget_exhausted`, `fail.rollback_anchor_missing`, `fail.terminal_publication_incomplete` |
| freeze/release | `deny.freeze.board_missing`, `deny.freeze.blocker_trace_missing`, `deny.freeze.signoff_forbidden` |
| material-first authoring | `deny.material.missing_archetype`, `deny.material.missing_surface_family`, `deny.material.missing_response_profile`, `deny.material.preview_blocked_incomplete_stack` |
| terrain/world authoring | `deny.terrain.layer_unresolved`, `deny.terrain.biome_overlay_illegal`, `deny.terrain.chunk_rebuild_invalid`, `deny.terrain.save_blocked_unresolved_truth`, `deny.world.validation_blocked_missing_material_truth` |
| sky/world context | `deny.sky.profile_invalid`, `deny.sky.binding_missing`, `deny.sky.preview_scope_invalid`, `deny.sky.visibility_missing` |

## Law
- denial and failure codes are stable ids, not cosmetic strings;
- editor labels may soften wording, but artifacts, packets, and route outcomes must retain canonical ids;
- no promoted day-zero action may fail with a vague generic error when a material-first denial family exists.


## Base-shell and advanced authoring denial additions
| Family | Canonical codes |
|---|---|
| shell/view | `disabled.surface_unavailable`, `deny.view.restore_context_missing`, `deny.view.selection_restore_illegal` |
| material presentation | `deny.material.microdetail_profile_missing`, `deny.material.weather_modulation_profile_missing`, `deny.material.preview_burn_illegal` |
| world validation | `deny.world.validation_blocked_chunk_integrity`, `deny.world.validation_blocked_environment_binding` |
| audio authoring | `deny.audio.emitter_class_illegal`, `deny.audio.zone_profile_missing`, `deny.audio.bus_policy_illegal`, `deny.audio.listener_profile_missing`, `deny.audio.occlusion_path_missing`, `deny.audio.voice_subtitle_relation_missing` |

## Material-centric denial additions
Add canonical denial families:
- `deny.material.response_family_missing`
- `deny.material.light_response_incompatible`
- `deny.material.visual_response_missing`
- `deny.material.acoustic_remap_illegal`
- `deny.material.cheap_runtime_rung_illegal`
- `deny.material.route_closure_incomplete`

## Workspace, viewport, extension, and backend-bridge codes
| Family | Canonical codes |
|---|---|
| workspace/window shell | `disabled.workspace_stage_unavailable`, `deny.window.layout_restore_invalid`, `deny.window.detach_scope_illegal`, `deny.window.layout_save_forbidden` |
| viewport shell | `deny.viewport.split_illegal_present_scope`, `deny.viewport.secondary_view_budget_exhausted`, `fail.backend.present_surface_unbound`, `fail.backend.shader_target_unavailable` |
| extension and assistant | `deny.extension.capability_not_granted`, `deny.extension.mount_scope_forbidden`, `deny.extension.bundle_signature_missing`, `deny.assistant.proposal_apply_forbidden`, `deny.assistant.revert_anchor_missing` |
| backend bridge | `fail.backend.bridge_port_missing`, `fail.backend.native_escape_hatch_illegal`, `fail.backend.platform_adapter_missing`, `fail.backend.capture_path_incomplete` |


## Dream-stack deep denial additions
| Family | Canonical codes |
|---|---|
| geodesy / far world | `deny.world.geo_scope_illegal`, `fail.world.rebase_unpublished`, `fail.world.far_causality_gap` |
| field substrate | `deny.field.scope_illegal`, `fail.field.conflict_unresolved`, `fail.field.replay_gap` |
| destruction / deformation | `deny.destruction.support_group_missing`, `fail.destruction.collapse_propagation_forbidden`, `fail.terrain.mass_ledger_fraud`, `deny.terrain.deformation_illegal` |
| hydrology / climate | `fail.hydrology.mass_balance_break`, `deny.hydrology.leak_geometry_illegal`, `fail.climate.front_identity_drift`, `deny.climate.arrival_window_illegal` |
| society / tactics / ecology | `fail.society.crime_consequence_missing`, `fail.tactics.cover_invalidation_gap`, `fail.ecology.pack_identity_drift` |
| wound / motion / dialogue | `deny.wound.body_topology_missing`, `fail.motion.contact_target_illegal`, `deny.dialogue.truth_mutation_forbidden`, `fail.dialogue.fallback_unavailable` |
| photoreal / audio / microgeometry / VFX | `fail.lighting.old_floor_budget_red`, `fail.audio.soundscape_continuity_gap`, `fail.microgeom.overdraw_red`, `fail.vfx.truth_separation_violation` |
