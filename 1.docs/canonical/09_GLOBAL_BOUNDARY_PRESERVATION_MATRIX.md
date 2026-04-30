# Global Boundary Preservation Matrix

**Stack version:** `SX-CANON/1.0.28/STACK-v34`

## Purpose
This document preserves the legal boundaries between packages in the StratumX canonical stack.
The crossing sets below are umbrella-level **closed sets**: anything not listed here and in the corresponding package roots is forbidden.

## Engine -> SDK Boundary

### Legal crossing classes
- `sdk::link_packets`
- `sdk::link_controls`
- `sdk::link_observations`
- `sdk::link_metrics`
- `sdk::compat_versions`
- `sdk::compat_capabilities`
- `sdk::compat_profiles`
- `sdk::compat_verdicts`
- `sdk::transport_policies`
- `sdk::legality_gates`
- `sdk::engine_session_handles`
- `sdk::engine_object_handles`
- `sdk::engine_runtime_handles`
- `sdk::engine_identity_refs`
- `sdk::engine_state_refs`
- `sdk::engine_artifact_refs`

### Forbidden crossings
- direct engine internal types;
- engine mutation interfaces;
- engine-private implementation details, ownership state, or raw pointers.

### Package-root anchors
- `sdk/13_TYPOLOGY_SYSTEM.md`
- `sdk/14_ROLE_CLASS_SEPARATION_MATRIX.md`
- `sdk/17_REF_SUBTYPES.md`
- `sdk/26_SHARED_TYPE_REGISTRY.md`
- `sdk/31_ENGINE_L4_BINDING_MAP.md`
- `sdk/33_HANDLE_AND_REF_OPACITY_LAW.md`

## SDK -> Tooling Boundary

### Legal crossing classes
- `tooling::authority_core`
- `tooling::command_envelopes`
- `tooling::transaction_ledger`
- `tooling::snapshot_plane`
- `tooling::index_plane`
- `tooling::derived_plane`
- `tooling::artifact_plane`
- `tooling::stream_plane`
- `tooling::cache_plane`
- `tooling::budget_runtime`
- `tooling::validation_runtime`
- `tooling::preview_runtime`
- `tooling::build_runtime`
- `tooling::release_runtime`
- `tooling::assistant_sessions`
- `tooling::context_evidence_packs`
- `tooling::proposal_runtime`
- `tooling::lowering_runtime`
- `tooling::safety_gates`
- `tooling::apply_revert_runtime`
- `tooling::assistant_ui_runtime`
- `tooling::model_request_runtime`

### Forbidden crossings
- direct engine truth intake without sdk mediation;
- sdk ownership leakage upward into tooling;
- tooling mutation of sdk bridge truth outside declared command/validation surfaces.

### Package-root anchors
- `tooling/13_DATA_PLANE_MODEL.md`
- `tooling/14_AUTHORITY_AND_TRANSACTION_MODEL.md`
- `tooling/15_SNAPSHOT_INDEX_DERIVED_MODEL.md`
- `tooling/16_L6A_ASSISTANT_RUNTIME_MODEL.md`
- `tooling/19_CROSS_LAYER_EXCHANGE_MODEL.md`
- `tooling/22_L5_SYNCHRONIZATION_MODEL.md`
- `tooling/26_SHARED_TYPE_REGISTRY.md`

## Tooling -> Editor Boundary

### Legal crossing classes
- `editor::shell_and_view_host`
- `editor::viewport_and_navigation`
- `editor::selection_and_interaction`
- `editor::tool_context_and_mode`
- `editor::panel_and_view`
- `editor::command_palette_and_shortcut`
- `editor::content_and_asset`
- `editor::outliner_world_browser`
- `editor::inspector_and_details`
- `editor::play_simulate_debug`
- `editor::assistant_surface`
- `editor::build_release_diagnostics_surface`
- `editor::domain_suites`
- `editor::pipeline_and_graph_services`
- `editor::extension_and_automation_services`
- `editor::collaboration_and_production_surfaces`

### Forbidden crossings
- direct editor reads of sdk or engine truth;
- editor reads of undeclared tooling internals;
- editor mutation of tooling state outside legal command channels.

### Package-root anchors
- `editor/13_PANEL_AND_VIEW_MODEL.md`
- `editor/15_CONTENT_AND_ASSET_MODEL.md`
- `editor/17_INSPECTOR_AND_DETAILS_MODEL.md`
- `editor/18_PLAY_SIMULATE_DEBUG_MODEL.md`
- `editor/20_BUILD_RELEASE_DIAGNOSTICS_SURFACE_MODEL.md`
- `editor/21_DOMAIN_SUITE_MODEL.md`
- `editor/22_EDITOR_DATAFLOW_AND_ACTIVATION_MODEL.md`
- `editor/31_SHARED_TYPE_REGISTRY.md`
- `editor/33_BOUNDARY_PRESERVATION_MATRIX.md`

## Boundary Integrity Rules
1. No package may read lower-stack truth outside the declared legal crossing classes.
2. No package may bypass the relay chain.
3. No package may gain ownership of another package's truth.
4. Every legal crossing class must be anchored both here and in the affected package root authorities.
5. Any omitted crossing class is forbidden until explicitly added here and in the affected package roots.
