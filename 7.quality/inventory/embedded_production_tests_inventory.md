# Embedded Production Tests Inventory

These files contain `#[cfg(test)]` or `#[test]` outside `7.quality` and must be migrated or explicitly allowed in a later phase.

| Path | Test kind | Decision |
|---|---|---|
| 4.tooling/l6.14-release-runtime/src/first_result_verification/evidence.rs | embedded unit tests | REVIEW_REQUIRED |
| 4.tooling/l6.14-release-runtime/src/first_result_verification/checks.rs | embedded unit tests | REVIEW_REQUIRED |
| 4.tooling/l6.0-authority-core/src/diagnostics.rs | embedded unit tests | ALLOW_MICRO_UNIT |
| 4.tooling/l6.0-authority-core/src/layer_purity.rs | embedded unit tests | ALLOW_MICRO_UNIT |
| 4.tooling/l6.0-authority-core/src/recovery/strategies.rs | embedded unit tests | ALLOW_MICRO_UNIT |
| 4.tooling/l6.0-authority-core/src/containers/material_authority_container.rs | embedded unit tests | ALLOW_MICRO_UNIT |
| 4.tooling/l6.0-authority-core/src/containers/audio/mod.rs | embedded unit tests | ALLOW_MICRO_UNIT |
| 4.tooling/l6.1-command-envelopes/src/canonical_command_schema/mod.rs | embedded unit tests | ALLOW_MICRO_UNIT |
| 4.tooling/l6.1-command-envelopes/src/validation/errors.rs | embedded unit tests | ALLOW_MICRO_UNIT |
| 4.tooling/l6.1-command-envelopes/src/validation/rules/mod.rs | embedded unit tests | ALLOW_MICRO_UNIT |
| 4.tooling/l6.1-command-envelopes/src/validation/rules/graphics.rs | embedded unit tests | ALLOW_MICRO_UNIT |
| 4.tooling/l6.1-command-envelopes/src/validation/rules/common.rs | embedded unit tests | ALLOW_MICRO_UNIT |
| 4.tooling/l6.1-command-envelopes/src/validation/rules/assets.rs | embedded unit tests | ALLOW_MICRO_UNIT |
| 4.tooling/l6.1-command-envelopes/src/promoted_commands/builders/mod.rs | embedded unit tests | ALLOW_MICRO_UNIT |
| 5.editor/l7.0-editor-command-spine/src/focus_routing.rs | embedded unit tests | REVIEW_REQUIRED |
| 5.editor/l7.0-editor-command-spine/src/focus_recovery.rs | embedded unit tests | REVIEW_REQUIRED |
| 5.editor/l7.0-editor-command-spine/src/editor_command_spine.rs | embedded unit tests | REVIEW_REQUIRED |
| 5.editor/l7.0-editor-command-spine/src/dispatch_material.rs | embedded unit tests | REVIEW_REQUIRED |
| 5.editor/l7.0-editor-command-spine/src/dispatch_audio.rs | embedded unit tests | REVIEW_REQUIRED |
| 5.editor/l7.0-editor-command-spine/src/canonical_action_api.rs | embedded unit tests | REVIEW_REQUIRED |
| 5.editor/l7.0-editor-command-spine/src/action_ids.rs | embedded unit tests | REVIEW_REQUIRED |
| 5.editor/l7.0-editor-command-spine/src/action_focus.rs | embedded unit tests | REVIEW_REQUIRED |
| 5.editor/l7.0-editor-command-spine/src/action_execution.rs | embedded unit tests | REVIEW_REQUIRED |
| 5.editor/l7.0-editor-command-spine/src/action_enums.rs | embedded unit tests | REVIEW_REQUIRED |
| 5.editor/l7.0-editor-command-spine/src/action_dispatch_core.rs | embedded unit tests | REVIEW_REQUIRED |
| 5.editor/l7.0-editor-command-spine/src/action_registry_types.rs | embedded unit tests | REVIEW_REQUIRED |
| 5.editor/l7.0-editor-command-spine/src/action_preconditions.rs | embedded unit tests | REVIEW_REQUIRED |
| 5.editor/l7.0-editor-command-spine/src/action_queries.rs | embedded unit tests | REVIEW_REQUIRED |
| 5.editor/l8.0-editor-shell/src/command_palette_state/query.rs | embedded unit tests | ALLOW_MICRO_UNIT |
| 5.editor/l8.0-editor-shell/src/command_palette_state/filters.rs | embedded unit tests | ALLOW_MICRO_UNIT |
| 5.editor/l8.0-editor-shell/src/command_spine/spine_lifecycle.rs | embedded unit tests | REVIEW_REQUIRED |
| 5.editor/l8.3-content-browser-system/src/content_browser_cache.rs | embedded unit tests | ALLOW_MICRO_UNIT |
| 5.editor/l8.5-tool-context-system/src/state_queries.rs | embedded unit tests | ALLOW_MICRO_UNIT |
| 5.editor/l8.5-tool-context-system/src/state_graph/mod.rs | embedded unit tests | ALLOW_MICRO_UNIT |
| 5.editor/l8.5-tool-context-system/src/session_state/mod.rs | embedded unit tests | ALLOW_MICRO_UNIT |
| 5.editor/l8.7-workspace-layout-system/src/queries/workspace_queries/persistence_queries.rs | embedded unit tests | ALLOW_MICRO_UNIT |
| 5.editor/l8.7-workspace-layout-system/src/queries/workspace_queries/panel_queries.rs | embedded unit tests | ALLOW_MICRO_UNIT |
| 5.editor/l8.7-workspace-layout-system/src/queries/workspace_queries/layout_queries.rs | embedded unit tests | ALLOW_MICRO_UNIT |
| 5.editor/l8.7-workspace-layout-system/src/persistence/workspace_persistence_view.rs | embedded unit tests | ALLOW_MICRO_UNIT |
| 5.editor/l8.10-diagnostics-surface/src/diagnostics_cache.rs | embedded unit tests | ALLOW_MICRO_UNIT |
| 5.editor/l9.0-world-authoring-suite/src/persistence/world_persistence_view/mod.rs | embedded unit tests | ALLOW_MICRO_UNIT |
| 5.editor/l9.3-material-lookdev-authoring-suite/src/material_cache.rs | embedded unit tests | ALLOW_MICRO_UNIT |
